use axum::{response::IntoResponse, http::StatusCode, Json};
use serde_json::json;

use crate::infra::repositories::error::RepoError;

#[derive(Debug)]
pub enum UserError {
    NotFound,
    DuplicateUsername,
    RepoError(RepoError),
}

impl IntoResponse for UserError {
    fn into_response(self) -> axum::response::Response {
        let (status, code, err_msg) = match self {
            Self::NotFound => (
                StatusCode::NOT_FOUND,
                20001,
                format!("User not found."),
            ),
            Self::DuplicateUsername => (
                StatusCode::BAD_REQUEST,
                20002,
                format!("Username already exists."),
            ),
            Self::RepoError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                20003,
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
