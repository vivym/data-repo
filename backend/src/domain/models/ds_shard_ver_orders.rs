use axum::{response::IntoResponse, http::StatusCode, Json};
use chrono::NaiveDateTime;
use serde_json::json;

use crate::AppError;

#[derive(Clone, Debug)]
pub struct DatasetShardVerificationOrderModel {
    pub id: i32,
    pub ds_id: i32,
    pub shard_id: i32,
    pub sample_id: i32,
    pub score: i32,
    pub comment: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug)]
pub enum DatasetShardVerificationOrderError {
    NotFound(i32),
    InternalError(AppError),
}

impl IntoResponse for DatasetShardVerificationOrderError {
    fn into_response(self) -> axum::response::Response {
        let (status, err_msg) = match self {
            Self::NotFound(id) => (
                StatusCode::NOT_FOUND,
                format!("DatasetShardVerificationOrderModel with id {} not found", id),
            ),
            Self::InternalError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal error: {}", err),
            ),
        };
        (
            status,
            Json(json!({
                "resource": "DatasetShardVerificationOrderModel",
                "message": err_msg,
                "happened_at": chrono::Utc::now(),
            })),
        )
            .into_response()
    }
}
