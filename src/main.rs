/*
 * Shuttle Christmas Code Hunt
 *      https://console.shuttle.rs/cch
 *
 * */

mod day1;
mod day4;
mod day5;
mod day6;
mod util;

use axum::{
    routing::{get, post},
    Router,
};
use day1::cube_bits;
use day4::{calculate_strength, compare_reindeer};
use day6::count_elfs;

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
        .route("/6", post(count_elfs));

    Ok(router.into())
}
