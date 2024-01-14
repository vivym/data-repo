use axum::extract::rejection::PathRejection;
use axum_macros::FromRequestParts;

use crate::error::AppError;

#[derive(FromRequestParts)]
#[from_request(via(axum::extract::Path), rejection(AppError))]
pub struct PathExtractor<T>(pub T);

impl From<PathRejection> for AppError {
    fn from(rejection: PathRejection) -> Self {
        Self::HttpPathParsingError(rejection.to_string())
    }
}
