/*
 * Shuttle Christmas Code Hunt
 *      https://console.shuttle.rs/cch
 *
 * */

mod day1;
mod day11;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod util;

use axum::{
    routing::{get, post},
    Router,
};
use day1::cube_bits;
use day11::get_no_of_red_pixels;
use day4::{calculate_strength, compare_reindeer};
use day6::count_elfs;
use day7::{bake_any, cookie_recipe};
use day8::{pokemon_momentum, pokemon_weight};
use tower_http::services::ServeDir;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/1/*num", get(cube_bits))
        .route("/4/strength", post(calculate_strength))
        .route("/4/contest", post(compare_reindeer))
        .route("/6", post(count_elfs))
        .route("/7/decode", get(cookie_recipe))
        .route("/7/bake", get(bake_any))
        .route("/8/weight/:pokemon_number", get(pokemon_weight))
        .route("/8/drop/:pokemon_number", get(pokemon_momentum))
        .route("/11/red_pixels", post(get_no_of_red_pixels))
        .nest_service("/11/assets", ServeDir::new("assets"));

    Ok(router.into())
}
