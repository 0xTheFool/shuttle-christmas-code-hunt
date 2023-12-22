use crate::AppState;
use axum::{
    debug_handler,
    extract::{
        ws::{Message, WebSocket},
        Path, State, WebSocketUpgrade,
    },
    response::Response,
};
use futures_util::{
    lock::Mutex,
    sink::SinkExt,
    stream::StreamExt,
};
use std::{collections::HashSet, sync::Arc};
use tokio::sync::broadcast;

#[debug_handler]
pub async fn websocket_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    let mut should_serve = false;
    while let Some(msg) = socket.recv().await {
        let msg = if let Ok(msg) = msg {
            msg
        } else {
            return;
        };

        match msg {
            Message::Text(s) => {
                if s == "serve" {
                    should_serve = true;
                }

                if should_serve && s == "ping" {
                    if socket
                        .send(Message::Text("pong".to_string()))
                        .await
                        .is_err()
                    {
                        return;
                    };
                }
            }
            _ => {}
        }
    }
}

#[derive(Debug, Clone)]
pub struct RoomState {
    user_set: HashSet<String>,
    tx: Arc<Mutex<broadcast::Sender<String>>>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
struct RevMsg {
    user: String,
    message: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
struct SendMsg {
    message: String,
}

impl RoomState {
    fn new() -> Self {
        Self {
            user_set: HashSet::new(),
            tx: Arc::new(Mutex::new(broadcast::channel(100).0)),
        }
    }
}

pub async fn reset_views(State(state): State<AppState>) {
    let mut data = state.total_views.lock().expect("Mutex was poisoned");
    *data = 0;
}

pub async fn get_views(State(state): State<AppState>) -> &'static str {
    format!("{}", state.total_views.lock().unwrap()).leak()
}

pub async fn websocket_handler_room(
    Path((room_no, user_id)): Path<(u64, String)>,
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> Response {
    let room_no = Arc::new(room_no);
    let user_id = Arc::new(user_id);
    ws.on_upgrade(|socket| handle_room(socket, state, room_no, user_id))
}

async fn handle_room(
    socket: WebSocket,
    state: AppState,
    room_no: Arc<u64>,
    user_id: Arc<String>,
) {
    let user_id = user_id.as_ref();
    let (mut sender, mut receiver) = socket.split();
    let mut tx = None;

    let mut is_user_id_taken = false;

    {
        let mut rooms = state.rooms.lock().expect("Mutex was poisoned");
        let room = rooms.entry(*room_no).or_insert_with(RoomState::new);
        tx = Some(room.tx.clone());

        if !room.user_set.contains(user_id) {
            room.user_set.insert(user_id.to_owned());
        } else {
            is_user_id_taken = true;
        }
    }

    if is_user_id_taken {
        sender
            .send(Message::Text(String::from("Username already taken.")))
            .await
            .unwrap();
        return;
    }

    let tx = tx.unwrap();
    let mut rx = tx.lock().await.subscribe();

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if let Ok(msg) = serde_json::from_str::<RevMsg>(&msg) {
                let msg = serde_json::to_string(&msg).unwrap();

                match sender.send(Message::Text(msg)).await {
                    Ok(_) => {
                        let mut total_view = state.total_views.lock().expect("Mutex was poisoned");
                        *total_view += 1;
                    }
                    Err(e) => {
                        println!("{:#?}",e);
                        break;
                    }
                }
            };
        }
    });

    let mut recv_task = {
        let tx = tx.clone();
        let user_id = user_id.clone();

        tokio::spawn(async move {
            while let Some(Ok(Message::Text(msg))) = receiver.next().await {
                if msg.len() > 128 {
                    continue;
                }

                match serde_json::from_str::<SendMsg>(&msg) {
                    Ok(msg) => {
                        let msg = RevMsg {
                            user: user_id.clone(),
                            message: msg.message,
                        };
                        let msg = serde_json::to_string(&msg).unwrap();

                        if let Err(e) = tx.lock().await.send(msg) {
                            println!("{e:#?}");
                            break;
                        };
                    }
                    Err(e) => {
                        println!("{e:#?}");
                    }
                }
                //tx.send(format!("{user_id}: {msg}")).unwrap();
            }
        })
    };

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };

    let mut rooms = state.rooms.lock().unwrap();
    rooms.get_mut(&room_no).unwrap().user_set.remove(user_id);
}
