use axum::{response::IntoResponse, http::StatusCode};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Tokio runtime error: {0}")]
    Tokio(#[from] std::io::Error),
    #[error("Axum webserver error: {0}")]
    Axum(#[from] axum::BoxError),
    #[error("Database pool error: {0}")]
    Pool(#[from] deadpool_diesel::PoolError),
    #[error("Database interact error: {0}")]
    Interact(#[from] deadpool_diesel::InteractError),
    #[error("Diesel error: {0}")]
    Diesel(#[from] diesel::result::Error),
    #[error("Http body parsing error: {0}")]
    HttpBodyParsingError(String),
    #[error("Http path parsing error: {0}")]
    HttpPathParsingError(String),
}

pub type AppResult<T> = Result<T, AppError>;

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, err_msg) = match self {
            Self::Tokio(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Tokio runtime error: {}", err),
            ),
            Self::Axum(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Axum webserver error: {}", err),
            ),
            Self::Pool(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database pool error: {}", err),
            ),
            Self::Interact(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database interact error: {}", err),
            ),
            Self::Diesel(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Diesel error: {}", err),
            ),
            Self::HttpBodyParsingError(err) => (
                StatusCode::BAD_REQUEST,
                format!("Http body parsing error: {}", err),
            ),
            Self::HttpPathParsingError(err) => (
                StatusCode::BAD_REQUEST,
                format!("Http path parsing error: {}", err),
            ),
        };
        (status, axum::Json(json!({ "message": err_msg }))).into_response()
    }
}
