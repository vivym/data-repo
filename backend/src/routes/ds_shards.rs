use axum::{Router, extract::{State, Query}, Json, routing::{post, get, delete}};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use utoipa::{ToSchema, IntoParams};

use crate::{
    domain::models::ds_shards::{DatasetShardError, DatasetShardModel},
    infra::repositories::ds_shards_repository::{self, NewDatasetShardDB, DatasetShardsFilter},
    server::AppState,
    utils::extractors::{
        json::JsonExtractor,
        path::PathExtractor,
    },
};

#[derive(Debug, Deserialize, ToSchema)]
pub struct DatasetShardCreationRequest {
    pub ds_id: i32,
    pub uri: String,
    pub num_samples: i32,
    pub verified: bool,
}

impl Into<NewDatasetShardDB> for DatasetShardCreationRequest {
    fn into(self) -> NewDatasetShardDB {
        NewDatasetShardDB {
            ds_id: self.ds_id,
            uri: self.uri,
            num_samples: self.num_samples,
            verified: self.verified,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DatasetShardResponse {
    id: i32,
    ds_id: i32,
    uri: String,
    num_samples: i32,
    verified: bool,
    #[schema(value_type = String)]
    created_at: NaiveDateTime,
    #[schema(value_type = String)]
    updated_at: NaiveDateTime,
}

impl From<DatasetShardModel> for DatasetShardResponse {
    fn from(ds_shard: DatasetShardModel) -> Self {
        Self {
            id: ds_shard.id,
            ds_id: ds_shard.ds_id,
            uri: ds_shard.uri,
            num_samples: ds_shard.num_samples,
            verified: ds_shard.verified,
            created_at: ds_shard.created_at,
            updated_at: ds_shard.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ListDatasetShardsResponse {
    shards: Vec<DatasetShardResponse>
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeleteDatasetShardResponse {
    done: bool
}

#[utoipa::path(
    post,
    path = "/v1/ds_shards",
    request_body = DatasetShardCreationRequest,
    responses(
        (status = 200, description = "Dataset shard created successfully", body = DatasetShardResponse),
    )
)]
#[instrument(skip(state))]
pub async fn create_ds_shard(
    State(state): State<AppState>,
    JsonExtractor(new_shard): JsonExtractor<DatasetShardCreationRequest>,
) -> Result<Json<DatasetShardResponse>, DatasetShardError> {
    let new_shard = new_shard.into();

    let created_shard = ds_shards_repository::create(
        &state.pg_pool, new_shard
    )
        .await
        .map_err(DatasetShardError::InternalError)?; // Convert AppError to DatasetShardError

    Ok(Json(DatasetShardResponse::from(created_shard)))
}

#[utoipa::path(
    get,
    path = "/v1/ds_shards/{id}",
    params(
        ("id", Path, description = "Dataset shard id")
    ),
    responses(
        (status = 200, description = "Dataset shard query successfully", body = DatasetShardResponse),
        (status = NOT_FOUND, description = "Dataset shard not found"),
    )
)]
#[instrument(skip(state))]
pub async fn get_ds_shard(
    State(state): State<AppState>,
    PathExtractor(ds_shard_id): PathExtractor<i32>,
) -> Result<Json<DatasetShardResponse>, DatasetShardError> {
    let shard = ds_shards_repository::get_by_id(
        &state.pg_pool, ds_shard_id
    )
        .await
        .map_err(DatasetShardError::InternalError)?; // Convert AppError to DatasetShardError

    Ok(Json(DatasetShardResponse::from(shard)))
}


#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct DatasetShardSearchQuery {
    /// Dataset id
    pub ds_id: Option<i32>,
    /// Skip, default: 0
    pub skip: Option<i64>,
    /// Limit, default: 20
    pub limit: Option<i64>,
}

#[utoipa::path(
    get,
    path = "/v1/ds_shards",
    params(DatasetShardSearchQuery),
    responses(
        (status = 200, description = "Dataset shard query successfully", body = ListDatasetShardsResponse),
        (status = NOT_FOUND, description = "Dataset shard not found"),
    )
)]
#[instrument(skip(state))]
pub async fn list_ds_shards(
    State(state): State<AppState>,
    Query(params): Query<DatasetShardsFilter>,
) -> Result<Json<ListDatasetShardsResponse>, DatasetShardError> {
    let shards = ds_shards_repository::get_all(
        &state.pg_pool, params
    )
        .await
        .map_err(DatasetShardError::InternalError)?; // Convert AppError to DatasetShardError

    let shards = shards
        .into_iter()
        .map(DatasetShardResponse::from)
        .collect();

    let shards = ListDatasetShardsResponse {
        shards
    };

    Ok(Json(shards))
}

#[utoipa::path(
    delete,
    path = "/v1/ds_shards/{id}",
    params(
        ("id", Path, description = "Dataset shard id")
    ),
    responses(
        (status = 200, description = "Dataset shard deletion successfully", body = DeleteDatasetShardResponse),
        (status = NOT_FOUND, description = "Dataset shard not found"),
    )
)]
#[instrument(skip(state))]
pub async fn delete_ds_shard(
    State(state): State<AppState>,
    PathExtractor(shard_id): PathExtractor<i32>,
) -> Result<Json<DeleteDatasetShardResponse>, DatasetShardError> {
    ds_shards_repository::delete_by_id(&state.pg_pool, shard_id)
        .await
        .map_err(DatasetShardError::InternalError)?; // Convert AppError to DatasetShardError

    let res = DeleteDatasetShardResponse {
        done: true
    };
    Ok(Json(res))
}

pub fn ds_shards_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", post(create_ds_shard))
        .route("/", get(list_ds_shards))
        .route("/:id", get(get_ds_shard))
        .route("/:id", delete(delete_ds_shard))
        .with_state(state)
}
