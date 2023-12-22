use axum::{http::StatusCode, response::IntoResponse};

#[derive(Debug)]
pub enum MyError {
    ParseError,
    SledRangeExceed,
    CookieNotProvided,
    InvalidBase64,
    InternalServerError,
    CustomError(String),
}

impl IntoResponse for MyError {
    fn into_response(self) -> axum::response::Response {
        let mut status_code = StatusCode::BAD_REQUEST;
        let body = match self {
            MyError::ParseError => "Error parsing Values",
            MyError::SledRangeExceed => "Number of params exceeded",
            MyError::CookieNotProvided => "Cookie Header Missing",
            MyError::InvalidBase64 => "Invalid Base64 Encoded String",
            MyError::CustomError(value) => value.leak(),
            MyError::InternalServerError => {
                status_code = StatusCode::INTERNAL_SERVER_ERROR;
                ""
            }
        };

        (status_code, body).into_response()
    }
}
