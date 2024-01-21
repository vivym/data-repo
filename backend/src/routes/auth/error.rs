use axum::{response::IntoResponse, http::StatusCode, Json};
use serde_json::json;

use crate::infra::repositories::error::RepoError;

#[derive(Debug)]
pub enum AuthError {
    InvalidCredentials,
    Unauthorized,
    UserNotActive,
    PermissionDenied,
    InvalidToken,
    InternalServerError(String),
    RepoError(RepoError),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let (status, code, err_msg) = match self {
            Self::InvalidCredentials => (
                StatusCode::BAD_REQUEST,
                10001,
                format!("Invalid credentials"),
            ),
            Self::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                10002,
                format!("Unauthorized"),
            ),
            Self::UserNotActive => (
                StatusCode::FORBIDDEN,
                10003,
                format!("User not active"),
            ),
            Self::PermissionDenied => (
                StatusCode::FORBIDDEN,
                10004,
                format!("Permission denied"),
            ),
            Self::InvalidToken => (
                StatusCode::UNAUTHORIZED,
                10005,
                format!("Invalid token"),
            ),
            Self::InternalServerError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                10006,
                format!("Internal server error: {}", msg),
            ),
            Self::RepoError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                10007,
                format!("Internal server error."),
            ),
        };
        (
            status,
            Json(json!({"code": code, "msg": err_msg})),
        )
            .into_response()
    }
}
