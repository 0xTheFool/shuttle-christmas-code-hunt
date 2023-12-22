use crate::{day13::create_schema, AppState};
use axum::{
    debug_handler,
    extract::{Path, State},
    Json,
};
use sqlx::FromRow;

#[debug_handler]
pub async fn create_schema_18(State(state): State<AppState>) {
    let mut query = "DROP TABLE IF EXISTS regions;";
    sqlx::query(query).execute(&state.pool).await.unwrap();

    create_schema(State(state.clone())).await;

    query = "CREATE TABLE regions (
  id INT PRIMARY KEY,
  name VARCHAR(50)
);";
    sqlx::query(query).execute(&state.pool).await.unwrap();
}

#[derive(Debug, FromRow, serde::Deserialize)]
pub struct Region {
    id: i64,
    name: String,
}

#[debug_handler]
pub async fn take_regions(State(state): State<AppState>, Json(regions): Json<Vec<Region>>) {
    let query = "INSERT INTO regions values ($1, $2)";

    for region in regions.iter() {
        sqlx::query(query)
            .bind(region.id)
            .bind(&region.name)
            .execute(&state.pool)
            .await
            .unwrap();
    }
}

#[derive(sqlx::FromRow, serde::Serialize)]
pub struct TotalResults {
    total: i64,
    region: String,
}

#[derive(sqlx::FromRow, serde::Serialize)]
pub struct TopListResults {
    region: String,
    top_gifts: Vec<String>,
}

#[debug_handler]
pub async fn total_regions(State(state): State<AppState>) -> Json<Vec<TotalResults>> {
    let query = "SELECT SUM(orders.quantity) as total,regions.name as region FROM orders 
      INNER JOIN regions ON orders.region_id = regions.id GROUP BY regions.name ORDER BY regions.name";
    let rows = sqlx::query_as::<_, TotalResults>(query)
        .fetch_all(&state.pool)
        .await
        .unwrap();

    Json(rows)
}

#[debug_handler]
pub async fn top_list(
    Path(number): Path<u32>,
    State(state): State<AppState>,
) -> Json<Vec<TopListResults>> {
    let query = format!(
        "
    SELECT 
        r.name as region,
        ARRAY_REMOVE(ARRAY_AGG(o.gift_name),NULL) as top_gifts
        FROM regions r
        LEFT JOIN LATERAL (
            SELECT o.gift_name,
            SUM(o.quantity) AS total_quantity
            FROM orders o
            WHERE o.region_id = r.id
            GROUP BY o.gift_name
            ORDER BY total_quantity DESC, o.gift_name ASC
            LIMIT {number}
        ) o ON TRUE
        GROUP BY r.name
        ORDER BY r.name ASC
        "
    );

    let rows = sqlx::query_as::<_, TopListResults>(&query)
        .fetch_all(&state.pool)
        .await
        .unwrap();

    Json(rows)
}
