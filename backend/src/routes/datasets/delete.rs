use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use tracing::instrument;
use utoipa::ToSchema;

use crate::{
    infra::repositories,
    server::AppState,
    utils::extractors::path::PathExtractor,
};
use super::error::DatasetError;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeleteDatasetResponse {
    pub code: i32,
    pub data: bool,
    pub msg: Option<String>,
}

#[utoipa::path(
    delete,
    path = "/v1/datasets/{id}",
    params(
        ("id", Path, description = "Dataset id")
    ),
    responses(
        (status = 200, description = "Dataset deletion successfully", body = DeleteDatasetResponse),
        (status = NOT_FOUND, description = "Dataset not found"),
    )
)]
#[instrument(skip(state))]
pub async fn delete_dataset(
    State(state): State<AppState>,
    PathExtractor(ds_id): PathExtractor<i32>,
) -> Result<Json<DeleteDatasetResponse>, DatasetError> {
    repositories::dataset::delete_by_id(&state.pg_pool, ds_id)
        .await
        .map_err(DatasetError::RepoError)?;

    Ok(Json(DeleteDatasetResponse {
        code: 0,
        data: true,
        msg: None,
    }))
}
