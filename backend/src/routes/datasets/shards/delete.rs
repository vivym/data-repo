use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use tracing::instrument;
use utoipa::ToSchema;

use crate::{
    infra::repositories,
    server::AppState,
    utils::extractors::path::PathExtractor,
};
use super::error::DatasetShardError;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeleteDatasetShardResponse {
    pub code: i32,
    pub data: bool,
    pub msg: Option<String>,
}

#[utoipa::path(
    delete,
    path = "/v1/datasets/shards/{id}",
    params(
        ("id", Path, description = "Dataset shard id")
    ),
    responses(
        (
            status = 200,
            description = "Dataset shard deletion successfully",
            body = DeleteDatasetShardResponse,
        ),
        (status = NOT_FOUND, description = "Dataset shard not found"),
    )
)]
#[instrument(skip(state))]
pub async fn delete_dataset_shard(
    State(state): State<AppState>,
    PathExtractor(shard_id): PathExtractor<i32>,
) -> Result<Json<DeleteDatasetShardResponse>, DatasetShardError> {
    repositories::ds_shard::delete_by_id(&state.pg_pool, shard_id)
        .await
        .map_err(DatasetShardError::RepoError)?;

    Ok(Json(DeleteDatasetShardResponse {
        code: 0,
        data: true,
        msg: None,
    }))
}
