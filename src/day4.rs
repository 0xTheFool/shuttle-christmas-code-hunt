use crate::util::MyError;
use axum::{debug_handler, response::Json};
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Reindeer {
    name: String,
    strength: u32,
    #[serde(default)]
    speed: f32,
    #[serde(default)]
    height: u32,
    #[serde(default)]
    antler_width: u32,
    #[serde(default)]
    snow_magic_power: u32,
    #[serde(default)]
    favorite_food: String,
    #[serde(default)]
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    candies_eaten_yesterday: u32,
}

#[debug_handler]
pub async fn calculate_strength(Json(payload): Json<Vec<Reindeer>>) -> Result<Json<u32>, MyError> {
    let sum = payload.iter().map(|r| r.strength).sum();
    Ok(Json(sum))
}

#[debug_handler]
pub async fn compare_reindeer(Json(payload): Json<Vec<Reindeer>>) -> Result<Json<Value>, MyError> {
    let mut fastest = Reindeer::default();
    let mut tallest = Reindeer::default();
    let mut magician = Reindeer::default();
    let mut consumer = Reindeer::default();

    for item in payload {
        if item.speed > fastest.speed {
            fastest = item.clone();
        }

        if item.height > tallest.height {
            tallest = item.clone();
        }

        if item.snow_magic_power > magician.snow_magic_power {
            magician = item.clone();
        }

        if item.candies_eaten_yesterday > consumer.candies_eaten_yesterday {
            consumer = item.clone();
        }
    }

    let result = Json(json!({
        "fastest": format!("Speeding past the finish line with a strength of {} is {}", fastest.strength, fastest.name) ,
        "tallest": format!("{} is standing tall with his {} cm wide antlers",tallest.name, tallest.antler_width),
        "magician": format!("{} could blast you away with a snow magic power of {}",magician.name,magician.snow_magic_power),
        "consumer": format!("{} ate lots of candies, but also some {}",consumer.name, consumer.favorite_food),
    }));

    Ok(result)
}
