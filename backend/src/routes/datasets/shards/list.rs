use axum::{extract::{State, Query}, Json};
use serde::{Deserialize, Serialize};
use tracing::instrument;
use utoipa::{ToSchema, IntoParams};

use crate::{
    infra::repositories::{self, ds_shard::DatasetShardsFilter},
    server::AppState,
};
use super::{error::DatasetShardError, schema::DatasetShardSchema};

#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct DatasetShardSearchQuery {
    /// Dataset ID
    pub ds_id: Option<i32>,
    /// Skip, default: 0
    pub skip: Option<i64>,
    /// Limit, default: 20
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ListDatasetShardsResponse {
    code: i32,
    data: Option<Vec<DatasetShardSchema>>,
    msg: Option<String>,
}

#[utoipa::path(
    get,
    path = "/v1/datasets/shards",
    params(DatasetShardSearchQuery),
    responses(
        (
            status = 200,
            description = "Dataset shard query successfully",
            body = ListDatasetShardsResponse,
        ),
        (status = NOT_FOUND, description = "Dataset shard not found"),
    )
)]
#[instrument(skip(state))]
pub async fn list_dataset_shards(
    State(state): State<AppState>,
    Query(params): Query<DatasetShardsFilter>,
) -> Result<Json<ListDatasetShardsResponse>, DatasetShardError> {
    let shards = repositories::ds_shard::get_all(
        &state.pg_pool, params
    )
        .await
        .map_err(DatasetShardError::RepoError)?;

    let shards = shards
        .into_iter()
        .map(DatasetShardSchema::from)
        .collect();

    Ok(Json(ListDatasetShardsResponse {
        code: 0,
        data: Some(shards),
        msg: None,
    }))
}
