use axum::{response::IntoResponse, http::StatusCode, Json};
use chrono::NaiveDateTime;
use serde_json::json;

use crate::AppError;

#[derive(Clone, Debug)]
pub struct DatasetModel {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug)]
pub enum DatasetError {
    NotFound(i32),
    InternalError(AppError),
}

impl IntoResponse for DatasetError {
    fn into_response(self) -> axum::response::Response {
        let (status, err_msg) = match self {
            Self::NotFound(id) => (
                StatusCode::NOT_FOUND,
                format!("DataseModel with id {} not found", id),
            ),
            Self::InternalError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal error: {}", err),
            ),
        };
        (
            status,
            Json(json!({
                "resource": "DatasetModel",
                "message": err_msg,
                "happened_at": chrono::Utc::now(),
            })),
        )
            .into_response()
    }
}
