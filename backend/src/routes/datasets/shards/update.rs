use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use tracing::instrument;
use utoipa::ToSchema;

use crate::{
    infra::repositories::{self, ds_shard::UpdatedDatasetShardDB},
    server::AppState,
    utils::extractors::{
        json::JsonExtractor,
        path::PathExtractor,
    },
};
use super::{error::DatasetShardError, schema::DatasetShardSchema};

#[derive(Debug, Deserialize, ToSchema)]
pub struct DatasetShardUpdateRequest {
    pub uri: Option<String>,
}

impl Into<UpdatedDatasetShardDB> for DatasetShardUpdateRequest {
    fn into(self) -> UpdatedDatasetShardDB {
        UpdatedDatasetShardDB {
            uri: self.uri,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DatasetShardUpdateResponse {
    pub code: i32,
    pub data: Option<DatasetShardSchema>,
    pub msg: Option<String>,
}

#[utoipa::path(
    put,
    path = "/v1/datasets/shards/{id}",
    request_body = DatasetShardUpdateRequest,
    responses(
        (
            status = 200,
            description = "Dataset shard update successfully",
            body = DatasetShardUpdateResponse,
        ),
    )
)]
#[instrument(skip(state))]
pub async fn update_dataset_shard(
    State(state): State<AppState>,
    PathExtractor(shard_id): PathExtractor<i32>,
    JsonExtractor(updated_shard): JsonExtractor<DatasetShardUpdateRequest>,
) -> Result<Json<DatasetShardUpdateResponse>, DatasetShardError> {
    repositories::ds_shard::try_get_by_id(
        &state.pg_pool, shard_id
    )
        .await
        .map_err(DatasetShardError::RepoError)?
        .ok_or(DatasetShardError::NotFound)?;

    let dataset = repositories::ds_shard::update_by_id(
        &state.pg_pool, shard_id, updated_shard.into()
    )
        .await
        .map_err(DatasetShardError::RepoError)?;

    Ok(Json(DatasetShardUpdateResponse {
        code: 0,
        data: Some(DatasetShardSchema::from(dataset)),
        msg: None,
    }))
}
