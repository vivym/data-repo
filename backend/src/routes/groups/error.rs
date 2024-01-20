use axum::{response::IntoResponse, http::StatusCode, Json};
use serde_json::json;

use crate::infra::repositories::error::RepoError;

#[derive(Debug)]
pub enum GroupError {
    NotFound,
    Duplicate,
    RepoError(RepoError),
}

impl IntoResponse for GroupError {
    fn into_response(self) -> axum::response::Response {
        let (status, code, err_msg) = match self {
            Self::NotFound => (
                StatusCode::NOT_FOUND,
                30001,
                format!("User not found."),
            ),
            Self::Duplicate => (
                StatusCode::BAD_REQUEST,
                30002,
                format!("Group already exists."),
            ),
            Self::RepoError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                30003,
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
