use crate::AppState;
use axum::{debug_handler, extract::State, Json};
use serde::Deserialize;
use serde_json::{json, Value};

#[debug_handler]
pub async fn sql_select(State(state): State<AppState>) -> String {
    let row: (i32,) = sqlx::query_as("SELECT $1")
        .bind(20231213_i32)
        .fetch_one(&state.pool)
        .await
        .unwrap();

    format!("{}", row.0)
}

#[derive(Deserialize, Clone)]
pub struct Order {
    id: i32,
    region_id: i32,
    gift_name: String,
    quantity: i32,
}

#[debug_handler]
pub async fn create_schema(State(state): State<AppState>) {
    let mut query = "DROP TABLE IF EXISTS orders;";
    sqlx::query(query).execute(&state.pool).await.unwrap();

    query = "CREATE TABLE orders (
  id INT PRIMARY KEY,
  region_id INT,
  gift_name VARCHAR(50),
  quantity INT
)";
    sqlx::query(query).execute(&state.pool).await.unwrap();
}

#[debug_handler]
pub async fn take_orders(State(state): State<AppState>, Json(orders): Json<Vec<Order>>) {
    let query = "INSERT INTO orders values ($1, $2, $3, $4)";

    for order in orders.iter() {
        sqlx::query(query)
            .bind(order.id)
            .bind(order.region_id)
            .bind(&order.gift_name)
            .bind(order.quantity)
            .execute(&state.pool)
            .await
            .unwrap();
    }
}

#[debug_handler]
pub async fn total_gifts(State(state): State<AppState>) -> Json<Value> {
    let query = "SELECT SUM(quantity) FROM orders";
    let row: (i64,) = sqlx::query_as(query).fetch_one(&state.pool).await.unwrap();

    Json(json!({
        "total": row.0
    }))
}

#[debug_handler]
pub async fn most_popular_gift(State(state): State<AppState>) -> Json<Value> {
    let query = "SELECT gift_name,SUM(quantity) as quantity FROM orders GROUP BY gift_name ORDER BY quantity DESC LIMIT 1";

    match sqlx::query_as::<_, (String,)>(query)
        .fetch_one(&state.pool)
        .await
    {
        Ok(a) => Json(json!({"popular": a.0} )),
        Err(_e) => Json(json!({"popular": Value::Null } )),
    }
}
