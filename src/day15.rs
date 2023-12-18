use crate::util::MyError;
use axum::http::StatusCode;
use axum::{debug_handler, extract::rejection::JsonRejection, response::IntoResponse, Json};
use regex::{Regex, RegexSet};
use reqwest::header;
use serde::Deserialize;
use serde_json::json;
use sha256::digest;

#[derive(Debug, Deserialize)]
pub struct Data {
    input: String,
}

#[debug_handler]
pub async fn validate(
    payload: Result<Json<Data>, JsonRejection>,
) -> Result<impl IntoResponse, MyError> {
    match payload {
        Ok(Json(payload)) => {
            let set = RegexSet::new(&[r"(.*[aeiouAEIOU].*){3,}", "ab|cd|pq|xy"]).unwrap();
            let matches = set.matches(&payload.input);

            if !matches.matched(1) {
                let re = Regex::new(r"([a-zA-Z]){2}").unwrap();
                let mut letter_appear_twirce = false;
                for (a, [_]) in re.captures_iter(&payload.input).map(|c| c.extract()) {
                    let chars: Vec<_> = a.chars().collect();
                    if chars[0] == chars[1] {
                        letter_appear_twirce = true;
                        break;
                    }
                }

                if matches.matched(0) && letter_appear_twirce {
                    let res = Json(json!({
                        "result": "nice",
                    }));

                    return Ok((
                        StatusCode::OK,
                        [(header::CONTENT_TYPE, "application/json")],
                        res,
                    ));
                }
            }

            let res = Json(json!({"result": "naughty"}));

            Ok((
                StatusCode::BAD_REQUEST,
                [(header::CONTENT_TYPE, "application/json")],
                res,
            ))
        }
        Err(err) => Err(MyError::CustomError(err.body_text())),
    }
}

#[debug_handler]
pub async fn game_of_the_year(
    payload: Result<Json<Data>, JsonRejection>,
) -> Result<impl IntoResponse, MyError> {
    match payload {
        Ok(Json(payload)) => {
            // Rule 1
            if payload.input.len() < 8 {
                return Ok(create_response(
                    "naughty",
                    "8 chars",
                    StatusCode::BAD_REQUEST,
                ));
            }

            let pattern = RegexSet::new(&[
                r"(.*[a-z].*){1,}",
                r"(.*[A-Z].*){1,}",
                r"(.*[0-9].*){1,}",
                r"(.*[0-9].*){5,}",
                r"(j.*o.*y)",
                r"(y.*j.*o.*y|o.*j.*o.*y|j.*o.*y.*o)",
                r"[\u2980-\u2BFF]",
                r"[\p{Emoji_Presentation}]",
            ])
            .unwrap();

            let matches = pattern.matches(&payload.input);

            // Rule 2
            if matches.matched(0) && matches.matched(1) && matches.matched(2) {
            } else {
                return Ok(create_response(
                    "naughty",
                    "more types of chars",
                    StatusCode::BAD_REQUEST,
                ));
            }

            // Rule 3
            if !matches.matched(3) {
                return Ok(create_response("naughty", "55555", StatusCode::BAD_REQUEST));
            }

            // Rule 4
            let pattern = Regex::new(r"(\d+)").unwrap();

            let mut sum = 0;
            for (_, [num]) in pattern.captures_iter(&payload.input).map(|c| c.extract()) {
                let num = num.parse::<u32>().map_err(|_| MyError::ParseError)?;
                sum += num;
            }

            if sum != 2023 {
                return Ok(create_response(
                    "naughty",
                    "math is hard",
                    StatusCode::BAD_REQUEST,
                ));
            }

            // Rule 5
            if !matches.matched(4) || matches.matched(5) {
                return Ok(create_response(
                    "naughty",
                    "not joyful enough",
                    StatusCode::NOT_ACCEPTABLE,
                ));
            }

            // Rule 6
            let chars: Vec<_> = payload
                .input
                .chars()
                .filter(char::is_ascii_alphabetic)
                .collect();

            let mut rule_6 = false;
            for j in 2..chars.len() {
                let i = j - 2;
                if chars[i] == chars[j] {
                    rule_6 = true;
                    break;
                }
            }

            if !rule_6 {
                return Ok(create_response(
                    "naughty",
                    "illegal: no sandwich",
                    StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS,
                ));
            }

            // Rule 7
            if !matches.matched(6) {
                return Ok(create_response(
                    "naughty",
                    "outranged",
                    StatusCode::RANGE_NOT_SATISFIABLE,
                ));
            }

            // Rule 8
            if !matches.matched(7) {
                return Ok(create_response(
                    "naughty",
                    "😳",
                    StatusCode::UPGRADE_REQUIRED,
                ));
            }

            // Rule 9
            let sha_digest = digest(payload.input);
            if !sha_digest.chars().last().is_some_and(|x| x == 'a') {
                return Ok(create_response(
                    "naughty",
                    "not a coffee brewer",
                    StatusCode::IM_A_TEAPOT,
                ));
            }

            Ok(create_response(
                "nice",
                "that's a nice password",
                StatusCode::OK,
            ))
        }
        Err(err) => Err(MyError::CustomError(err.body_text())),
    }
}

fn create_response(result: &str, reason: &str, status_code: StatusCode) -> impl IntoResponse {
    let res = json!({
                "result": result,
                "reason": reason,
    });

    (
        status_code,
        [(header::CONTENT_TYPE, "application/json")],
        Json(res),
    )
}
