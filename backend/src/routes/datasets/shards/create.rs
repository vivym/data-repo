use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use tracing::instrument;
use utoipa::ToSchema;

use crate::{
    infra::repositories::{self, ds_shard::NewDatasetShardDB},
    server::AppState,
    utils::extractors::json::JsonExtractor,
};
use super::{error::DatasetShardError, schema::DatasetShardSchema};

#[derive(Debug, Deserialize, ToSchema)]
pub struct DatasetShardCreationRequest {
    pub uri: String,
}

impl Into<NewDatasetShardDB> for DatasetShardCreationRequest {
    fn into(self) -> NewDatasetShardDB {
        NewDatasetShardDB {
            uri: self.uri,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DatasetShardCreationResponse {
    pub code: i32,
    pub data: Option<DatasetShardSchema>,
    pub msg: Option<String>,
}

#[utoipa::path(
    post,
    path = "/v1/datasets/shards",
    request_body = DatasetShardCreationRequest,
    responses(
        (
            status = 200,
            description = "Dataset shard created successfully",
            body = DatasetShardCreationResponse,
        ),
    )
)]
#[instrument(skip(state))]
pub async fn create_dataset_shard(
    State(state): State<AppState>,
    JsonExtractor(new_shard): JsonExtractor<DatasetShardCreationRequest>,
) -> Result<Json<DatasetShardCreationResponse>, DatasetShardError> {
    let shard_in_db = repositories::ds_shard::try_get_by_uri(
        &state.pg_pool, new_shard.uri.clone()
    )
        .await
        .map_err(DatasetShardError::RepoError)?;

    if shard_in_db.is_some() {
        return Err(DatasetShardError::Duplicate);
    }

    let created_shard = repositories::ds_shard::create(
        &state.pg_pool, new_shard.into()
    )
        .await
        .map_err(DatasetShardError::RepoError)?;

    Ok(Json(DatasetShardCreationResponse {
        code: 0,
        data: Some(DatasetShardSchema::from(created_shard)),
        msg: None,
    }))
}
