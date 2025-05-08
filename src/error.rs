use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub type Result<T> = core::result::Result<T,Error>;

#[derive(Debug)]
pub enum Error {
    NotFound,
    LoginFail,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> error = {self:?}");

        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
    }

}