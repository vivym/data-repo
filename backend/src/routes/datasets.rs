use axum::{Router, extract::State, Json, routing::{post, get}};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use utoipa::ToSchema;

use crate::{
    domain::models::datasets::{DatasetError, DatasetModel},
    infra::repositories::dataset_repository::{self, NewDatasetDB},
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

    let created_dataset = dataset_repository::create_dataset(
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
        ("id", description = "Dataset id")
    ),
    responses(
        (status = 200, description = "Dataset query successfully", body = DatasetResponse),
        (status = 404, description = "Dataset not found"),
    )
)]
#[instrument(skip(state))]
pub async fn get_dataset_by_id(
    State(state): State<AppState>,
    PathExtractor(dataset_id): PathExtractor<i32>,
) -> Result<Json<DatasetResponse>, DatasetError> {
    let dataset = dataset_repository::get_dataset_by_id(
        &state.pg_pool, dataset_id
    )
        .await
        .map_err(DatasetError::InternalError)?; // Convert AppError to DatasetError

    Ok(Json(DatasetResponse::from(dataset)))
}

pub fn datasets_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", post(create_dataset))
        .route("/:id", get(get_dataset_by_id))
        .with_state(state)
}
