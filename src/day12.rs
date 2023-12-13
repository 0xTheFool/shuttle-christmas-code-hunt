use crate::util::MyError;
use axum::extract::{Path, State};
use axum::{debug_handler, Json};
use chrono::prelude::*;
use chrono::{DateTime, Utc};
use serde_json::{json, Value};
use std::time::SystemTime;
use ulid::Ulid;
use uuid::Uuid;
use crate::AppState;


#[debug_handler]
pub async fn save_string(Path(string): Path<String>, State(state): State<AppState>) {
    let time = SystemTime::now();
    let mut packets = state.packets.lock().expect("Mutex was poisoned");

    packets.insert(string, time);
}

#[debug_handler]
pub async fn load_string(
    Path(string): Path<String>,
    State(state): State<AppState>,
) -> Result<String, MyError> {
    let time = SystemTime::now();
    let packets = state.packets.lock().expect("Mutex was poisoned");
    if let Some(last_accessed) = packets.get(&string) {
        let duration = time.duration_since(*last_accessed).unwrap();

        let result = format!("{}", duration.as_secs());
        Ok(result)
    } else {
        Err(MyError::CustomError(
            "Given String was not saved".to_string(),
        ))
    }
}

#[debug_handler]
pub async fn convert_ulids_to_uuids(Json(ulids): Json<Vec<String>>) -> Json<Vec<String>> {
    let uuids = ulids
        .iter()
        .rev()
        .map(|x| {
            let ulid = Ulid::from_string(x).unwrap();

            Uuid::from_bytes(ulid.to_bytes()).to_string()
        })
        .collect::<Vec<String>>();

    Json(uuids)
}

#[debug_handler]
pub async fn get_ulids(
    Path(given_week_day): Path<u32>,
    Json(ulids_array): Json<Vec<String>>,
) -> Json<Value> {
    let mut christmas_eve = 0;
    let mut weekday = 0;
    let mut in_the_future = 0;
    let mut lsb_is_1 = 0;

    ulids_array
        .iter()
        .map(|x| Ulid::from_string(x).unwrap())
        .for_each(|x| {
            let date: DateTime<Utc> = x.datetime().into();

            if date.day() == 24 && date.month() == 12 {
                christmas_eve += 1;
            }
            if date.weekday().number_from_monday() == given_week_day + 1 {
                weekday += 1;
            }
            if date > Utc::now() {
                in_the_future += 1;
            }
            if x.0 & 1u128 == 1 {
                lsb_is_1 += 1;
            }
        });

    Json(json!({
        "christmas eve": christmas_eve,
        "weekday": weekday,
        "in the future": in_the_future,
        "LSB is 1": lsb_is_1
    }))
}
