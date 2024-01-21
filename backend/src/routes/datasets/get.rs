use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use tracing::instrument;
use utoipa::ToSchema;

use crate::{
    infra::repositories,
    server::AppState,
    utils::extractors::path::PathExtractor,
};
use super::{error::DatasetError, schema::DatasetSchema};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GetDatasetResponse {
    pub code: i32,
    pub data: Option<DatasetSchema>,
    pub msg: Option<String>,
}

#[utoipa::path(
    get,
    path = "/v1/datasets/{id}",
    params(
        ("id", Path, description = "Dataset id")
    ),
    responses(
        (status = 200, description = "Dataset query successfully", body = GetDatasetResponse),
        (status = NOT_FOUND, description = "Dataset not found"),
    )
)]
#[instrument(skip(state))]
pub async fn get_dataset(
    State(state): State<AppState>,
    PathExtractor(ds_id): PathExtractor<i32>,
) -> Result<Json<GetDatasetResponse>, DatasetError> {
    let ds = repositories::dataset::get_by_id(
        &state.pg_pool, ds_id
    )
        .await
        .map_err(DatasetError::RepoError)?;

    Ok(Json(GetDatasetResponse {
        code: 0,
        data: Some(DatasetSchema::from(ds)),
        msg: None,
    }))
}
