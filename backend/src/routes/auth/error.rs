use axum::{response::IntoResponse, http::StatusCode, Json};
use serde_json::json;

#[derive(Debug)]
pub enum AuthError {
    InvalidCredentials,
    InternalServerError(String),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let (status, code, err_msg) = match self {
            Self::InvalidCredentials => (
                StatusCode::UNAUTHORIZED,
                10001,
                format!("Invalid credentials"),
            ),
            Self::InternalServerError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                10002,
                format!("Internal server error: {}", msg),
            ),
        };
        (
            status,
            Json(json!({"code": code, "msg": err_msg})),
        )
            .into_response()
    }
}
