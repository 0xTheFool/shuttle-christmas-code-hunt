use axum::{http::StatusCode, response::IntoResponse};

pub enum MyError {
    ParseError,
    SledRangeExceed,
}

impl IntoResponse for MyError {
    fn into_response(self) -> axum::response::Response {
        let body = match self {
            MyError::ParseError => "Error parsing Values",
            MyError::SledRangeExceed => "Number of params exceeded",
        };

        (StatusCode::BAD_REQUEST, body).into_response()
    }
}
