use axum::{http::StatusCode, response::IntoResponse};

pub type Result<T> = core::result::Result<T, Error>;
use axum::response::Response;

#[derive(Debug)]
pub enum Error {
    Loginfail,
    TicketDeleteFailIdNotFound { id: u64 },
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> Error: {:<12} - {self:?}", "INTO_RES");

        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED SERVER ERROR").into_response()
    }
}
