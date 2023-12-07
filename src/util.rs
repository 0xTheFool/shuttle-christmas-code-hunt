use axum::{http::StatusCode, response::IntoResponse};

pub enum MyError {
    ParseError,
    SledRangeExceed,
    CookieNotProvided,
    InvalidBase64,
}

impl IntoResponse for MyError {
    fn into_response(self) -> axum::response::Response {
        let body = match self {
            MyError::ParseError => "Error parsing Values",
            MyError::SledRangeExceed => "Number of params exceeded",
            MyError::CookieNotProvided => "Cookie Header Missing",
            MyError::InvalidBase64 => "Invalid Base64 Encoded String",
        };

        (StatusCode::BAD_REQUEST, body).into_response()
    }
}
