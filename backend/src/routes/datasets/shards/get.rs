use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use tracing::instrument;
use utoipa::ToSchema;

use crate::{
    infra::repositories,
    server::AppState,
    utils::extractors::path::PathExtractor,
};
use super::{error::DatasetShardError, schema::DatasetShardSchema};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GetDatasetShardResponse {
    pub code: i32,
    pub data: Option<DatasetShardSchema>,
    pub msg: Option<String>,
}

#[utoipa::path(
    get,
    path = "/v1/datasets/shards/{id}",
    params(
        ("id", Path, description = "Dataset shard id")
    ),
    responses(
        (
            status = 200,
            description = "Dataset shard query successfully",
            body = GetDatasetShardResponse,
        ),
        (status = NOT_FOUND, description = "Dataset shard not found"),
    )
)]
#[instrument(skip(state))]
pub async fn get_dataset_shard(
    State(state): State<AppState>,
    PathExtractor(shard_id): PathExtractor<i32>,
) -> Result<Json<GetDatasetShardResponse>, DatasetShardError> {
    let shard = repositories::ds_shard::get_by_id(
        &state.pg_pool, shard_id
    )
        .await
        .map_err(DatasetShardError::RepoError)?;

    Ok(Json(GetDatasetShardResponse {
        code: 0,
        data: Some(DatasetShardSchema::from(shard)),
        msg: None,
    }))
}
