use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use tracing::instrument;
use utoipa::ToSchema;

use crate::{
    infra::repositories,
    server::AppState,
    utils::extractors::path::PathExtractor,
};
use super::error::DatasetItemError;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeleteDatasetItemResponse {
    pub code: i32,
    pub data: bool,
    pub msg: Option<String>,
}

#[utoipa::path(
    delete,
    path = "/v1/datasets/items/{id}",
    params(
        ("id", Path, description = "Dataset item id")
    ),
    responses(
        (
            status = 200,
            description = "Dataset item deletion successfully",
            body = DeleteDatasetItemResponse,
        ),
        (status = NOT_FOUND, description = "Dataset item not found"),
    )
)]
#[instrument(skip(state))]
pub async fn delete_dataset_item(
    State(state): State<AppState>,
    PathExtractor(item_id): PathExtractor<i32>,
) -> Result<Json<DeleteDatasetItemResponse>, DatasetItemError> {
    repositories::ds_item::delete_by_id(&state.pg_pool, item_id)
        .await
        .map_err(DatasetItemError::RepoError)?;

    Ok(Json(DeleteDatasetItemResponse {
        code: 0,
        data: true,
        msg: None,
    }))
}
