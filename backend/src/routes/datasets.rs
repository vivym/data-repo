use axum::{Router, extract::{State, Query}, Json, routing::{post, get, delete}};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use utoipa::{ToSchema, IntoParams};

use crate::{
    domain::models::datasets::{DatasetError, DatasetModel},
    infra::repositories::dataset_repository::{self, NewDatasetDB, DatasetsFilter},
    server::AppState,
    utils::extractors::{
        json::JsonExtractor,
        path::PathExtractor,
    },
};

#[derive(Debug, Deserialize, ToSchema)]
pub struct DatasetCreationRequest {
    name: String,
    description: String,
}

impl Into<NewDatasetDB> for DatasetCreationRequest {
    fn into(self) -> NewDatasetDB {
        NewDatasetDB {
            name: self.name,
            description: self.description,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DatasetResponse {
    id: i32,
    name: String,
    description: String,
    #[schema(value_type = String)]
    created_at: NaiveDateTime,
    #[schema(value_type = String)]
    updated_at: NaiveDateTime,
}

impl From<DatasetModel> for DatasetResponse {
    fn from(dataset: DatasetModel) -> Self {
        Self {
            id: dataset.id,
            name: dataset.name,
            description: dataset.description,
            created_at: dataset.created_at,
            updated_at: dataset.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ListDatasetsResponse {
    datasets: Vec<DatasetResponse>
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeleteDatasetResponse {
    done: bool
}

#[utoipa::path(
    post,
    path = "/v1/datasets",
    request_body = DatasetCreationRequest,
    responses(
        (status = 200, description = "Dataset created successfully", body = DatasetResponse),
    )
)]
#[instrument(skip(state))]
pub async fn create_dataset(
    State(state): State<AppState>,
    JsonExtractor(new_dataset): JsonExtractor<DatasetCreationRequest>,
) -> Result<Json<DatasetResponse>, DatasetError> {
    let new_dataset = new_dataset.into();

    let created_dataset = dataset_repository::create(
        &state.pg_pool, new_dataset
    )
        .await
        .map_err(DatasetError::InternalError)?; // Convert AppError to DatasetError

    Ok(Json(DatasetResponse::from(created_dataset)))
}

#[utoipa::path(
    get,
    path = "/v1/datasets/{id}",
    params(
        ("id", Path, description = "Dataset id")
    ),
    responses(
        (status = 200, description = "Dataset query successfully", body = DatasetResponse),
        (status = NOT_FOUND, description = "Dataset not found"),
    )
)]
#[instrument(skip(state))]
pub async fn get_dataset(
    State(state): State<AppState>,
    PathExtractor(dataset_id): PathExtractor<i32>,
) -> Result<Json<DatasetResponse>, DatasetError> {
    let dataset = dataset_repository::get_by_id(
        &state.pg_pool, dataset_id
    )
        .await
        .map_err(DatasetError::InternalError)?; // Convert AppError to DatasetError

    Ok(Json(DatasetResponse::from(dataset)))
}


#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct DatasetSearchQuery {
    /// Skip, default: 0
    #[serde(default = "default_skip")]
    pub skip: i64,
    /// Limit, default: 20
    #[serde(default = "default_limit")]
    pub limit: i64,
}

fn default_skip() -> i64 {
    0
}

fn default_limit() -> i64 {
    20
}

#[utoipa::path(
    get,
    path = "/v1/datasets",
    params(DatasetSearchQuery),
    responses(
        (status = 200, description = "Dataset query successfully", body = ListDatasetsResponse),
        (status = NOT_FOUND, description = "Dataset not found"),
    )
)]
#[instrument(skip(state))]
pub async fn list_datasets(
    State(state): State<AppState>,
    Query(params): Query<DatasetsFilter>,
) -> Result<Json<ListDatasetsResponse>, DatasetError> {
    let datasets = dataset_repository::get_all(
        &state.pg_pool, params
    )
        .await
        .map_err(DatasetError::InternalError)?; // Convert AppError to DatasetError

    let datasets = datasets
        .into_iter()
        .map(DatasetResponse::from)
        .collect();

    let datasets = ListDatasetsResponse {
        datasets
    };

    Ok(Json(datasets))
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
    PathExtractor(dataset_id): PathExtractor<i32>,
) -> Result<Json<DeleteDatasetResponse>, DatasetError> {
    dataset_repository::delete_by_id(&state.pg_pool, dataset_id)
        .await
        .map_err(DatasetError::InternalError)?; // Convert AppError to DatasetError

    let res = DeleteDatasetResponse {
        done: true
    };
    Ok(Json(res))
}

pub fn datasets_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", post(create_dataset))
        .route("/", get(list_datasets))
        .route("/:id", get(get_dataset))
        .route("/:id", delete(delete_dataset))
        .with_state(state)
}
