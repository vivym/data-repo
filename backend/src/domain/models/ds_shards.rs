use axum::{response::IntoResponse, http::StatusCode, Json};
use chrono::NaiveDateTime;
use serde_json::json;

use crate::AppError;

#[derive(Clone, Debug)]
pub struct DatasetShardModel {
    pub id: i32,
    pub ds_id: i32,
    pub uri: String,
    pub num_samples: i32,
    pub verified: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug)]
pub enum DatasetShardError {
    NotFound(i32),
    InternalError(AppError),
}

impl IntoResponse for DatasetShardError {
    fn into_response(self) -> axum::response::Response {
        let (status, err_msg) = match self {
            Self::NotFound(id) => (
                StatusCode::NOT_FOUND,
                format!("DataseShardModel with id {} not found", id),
            ),
            Self::InternalError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal error: {}", err),
            ),
        };
        (
            status,
            Json(json!({
                "resource": "DataseShardModel",
                "message": err_msg,
                "happened_at": chrono::Utc::now(),
            })),
        )
            .into_response()
    }
}
