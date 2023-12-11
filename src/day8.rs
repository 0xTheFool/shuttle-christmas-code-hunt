use crate::util::MyError;
use axum::{debug_handler, extract::Path};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Pokemon {
    weight: f32,
}

#[debug_handler]
pub async fn pokemon_weight(Path(pokemon_number): Path<u32>) -> Result<String, MyError> {
    let url = format!("https://pokeapi.co/api/v2/pokemon/{pokemon_number}");

    match reqwest::get(url).await {
        Ok(response) => match response.json::<Pokemon>().await {
            Ok(res) => Ok(format!("{}", res.weight / 10.0)),
            Err(e) => Err(MyError::CustomError(e.to_string())),
        },
        Err(e) => Err(MyError::CustomError(e.to_string())),
    }
}

#[debug_handler]
pub async fn pokemon_momentum(Path(pokemon_number): Path<u32>) -> Result<String, MyError> {
    let wieght = match pokemon_weight(Path(pokemon_number)).await {
        Ok(value) => match value.parse::<f32>() {
            Ok(value) => value,
            Err(_) => {
                return Err(MyError::ParseError);
            }
        },
        Err(e) => {
            return Err(e);
        }
    };

    let final_velocity = (2f32 * 9.825 * 10f32).sqrt();
    let momentum = wieght * final_velocity;

    Ok(format!("{momentum}"))
}
