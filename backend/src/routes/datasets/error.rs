use axum::{response::IntoResponse, http::StatusCode, Json};
use serde_json::json;

use crate::infra::repositories::error::RepoError;

#[derive(Debug)]
pub enum DatasetError {
    NotFound,
    Duplicate,
    RepoError(RepoError),
}

impl IntoResponse for DatasetError {
    fn into_response(self) -> axum::response::Response {
        let (status, code, err_msg) = match self {
            Self::NotFound => (
                StatusCode::NOT_FOUND,
                40001,
                format!("Dataset not found."),
            ),
            Self::Duplicate => (
                StatusCode::BAD_REQUEST,
                40002,
                format!("Dataset already exists."),
            ),
            Self::RepoError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                40003,
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
