use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;

pub enum Response<T: Serialize> {
    Success(T),
    Err(StatusCode, Option<String>),
}

impl<T: Serialize> IntoResponse for Response<T> {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Success(value) => (StatusCode::OK, Json(value)).into_response(),
            Self::Err(status_code, message) => (
                status_code,
                match message {
                    Some(message) => message,
                    None => String::from("Internal server error"),
                },
            )
                .into_response(),
        }
    }
}
