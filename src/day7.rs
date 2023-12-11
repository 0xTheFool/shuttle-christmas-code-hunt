use std::collections::HashMap;

use crate::util::MyError;
use axum::{
    debug_handler,
    http::{header::COOKIE, HeaderMap},
    Json,
};
use base64::{engine::general_purpose, Engine};
use serde::Deserialize;
use serde::Serialize;
use serde_json::{json, Value};

#[debug_handler]
pub async fn cookie_recipe(headers: HeaderMap) -> Result<String, MyError> {
    if let Some(cookie_data) = headers.get(COOKIE) {
        let cookie_data = cookie_data.to_str().unwrap().to_string();

        let recipe = &cookie_data["recipe=".len()..];

        if let Ok(bytes) = general_purpose::STANDARD.decode(recipe) {
            let result = String::from_utf8_lossy(&bytes[..]);

            Ok(result.to_string())
        } else {
            Err(MyError::InvalidBase64)
        }
    } else {
        Err(MyError::CookieNotProvided)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct AnyRecipe {
    recipe: HashMap<String, u64>,
    pantry: HashMap<String, u64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AnyCookie {
    cookies: u64,
    pantry: HashMap<String, u64>,
}

#[debug_handler]
pub async fn bake_any(headers: HeaderMap) -> Result<Json<Value>, MyError> {
    if let Some(cookie_data) = headers.get(COOKIE) {
        let cookie_data = cookie_data.to_str().unwrap().to_string();

        let recipe = &cookie_data["recipe=".len()..];

        if let Ok(bytes) = general_purpose::STANDARD.decode(recipe) {
            let decoded_data: AnyRecipe = serde_json::from_slice(&bytes[..]).unwrap();

            let recipe = decoded_data.recipe;
            let mut pantry = decoded_data.pantry;

            let cookies = recipe
                .iter()
                .map(|(k, v)| {
                    if let Some(value) = pantry.get(k) {
                        if *v != 0 {
                            value / v
                        } else {
                            u64::MAX
                        }
                    } else {
                        u64::MAX
                    }
                })
                .min()
                .unwrap();

            for (k, v) in pantry.iter_mut() {
                if let Some(value) = recipe.get(k) {
                    *v = *v - value * cookies;
                }
            }

            let result = json!(AnyCookie { cookies, pantry });

            Ok(Json(result))
        } else {
            Err(MyError::InvalidBase64)
        }
    } else {
        Err(MyError::CookieNotProvided)
    }
}
