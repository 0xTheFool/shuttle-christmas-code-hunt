/*
 * Shuttle Christmas Code Hunt
 *      https://console.shuttle.rs/cch
 *
 * */

mod day1;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod util;

use std::{collections::HashMap, sync::{Arc, Mutex}, time::SystemTime, path::PathBuf};
use axum::{
    routing::{get, post},
    Router,
};
use day1::cube_bits;
use day11::get_no_of_red_pixels;
use day14::{render_html_unsafe, render_html_safe};
use day15::{validate, game_of_the_year};
use day4::{calculate_strength, compare_reindeer};
use day6::count_elfs;
use day7::{bake_any, cookie_recipe};
use day8::{pokemon_momentum, pokemon_weight};
use day12::{load_string,save_string, convert_ulids_to_uuids,get_ulids};
use day13::{sql_select, create_schema, take_orders, total_gifts, most_popular_gift};
use sqlx::PgPool;
use tower_http::services::ServeDir;
use util::MyError;


#[derive(Debug, Clone)]
pub struct AppState {
    pub packets: Arc<Mutex<HashMap<String, SystemTime>>>,
    pub pool: PgPool,
}

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn get_error() -> MyError {
    MyError::InternalServerError
}

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {

    let state = AppState {
        packets: Arc::new(Mutex::new(HashMap::new())),
        pool,
    };

    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(get_error))
        .route("/1/*num", get(cube_bits))
        .route("/4/strength", post(calculate_strength))
        .route("/4/contest", post(compare_reindeer))
        .route("/6", post(count_elfs))
        .route("/7/decode", get(cookie_recipe))
        .route("/7/bake", get(bake_any))
        .route("/8/weight/:pokemon_number", get(pokemon_weight))
        .route("/8/drop/:pokemon_number", get(pokemon_momentum))
        .route("/11/red_pixels", post(get_no_of_red_pixels))
        .route("/12/save/:string", post(save_string))
        .route("/12/load/:string", get(load_string))
        .route("/12/ulids", post(convert_ulids_to_uuids))
        .route("/12/ulids/:weekday", post(get_ulids))
        .route("/13/sql", get(sql_select))
        .route("/13/reset", post(create_schema))
        .route("/13/orders", post(take_orders))
        .route("/13/orders/total", get(total_gifts))
        .route("/13/orders/popular", get(most_popular_gift))
        .route("/14/unsafe",post(render_html_unsafe))
        .route("/14/safe",post(render_html_safe))
        .route("/15/nice", post(validate) )
        .route("/15/game", post(game_of_the_year))
        .nest_service("/11/assets", ServeDir::new(PathBuf::from("assets")))
        .with_state(state);

    Ok(router.into())
}
