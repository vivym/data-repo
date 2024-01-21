use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use tracing::instrument;
use utoipa::ToSchema;

use crate::{
    infra::repositories,
    server::AppState,
    utils::extractors::path::PathExtractor,
};
use super::{error::DatasetItemError, schema::DatasetItemSchema};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GetDatasetItemResponse {
    pub code: i32,
    pub data: Option<DatasetItemSchema>,
    pub msg: Option<String>,
}

#[utoipa::path(
    get,
    path = "/v1/datasets/items/{id}",
    params(
        ("id", Path, description = "Dataset item id")
    ),
    responses(
        (
            status = 200,
            description = "Dataset item query successfully",
            body = GetDatasetItemResponse,
        ),
        (status = NOT_FOUND, description = "Dataset item not found"),
    )
)]
#[instrument(skip(state))]
pub async fn get_dataset_item(
    State(state): State<AppState>,
    PathExtractor(item_id): PathExtractor<i32>,
) -> Result<Json<GetDatasetItemResponse>, DatasetItemError> {
    let item = repositories::ds_item::get_by_id(
        &state.pg_pool, item_id
    )
        .await
        .map_err(DatasetItemError::RepoError)?;

    Ok(Json(GetDatasetItemResponse {
        code: 0,
        data: Some(DatasetItemSchema::from(item)),
        msg: None,
    }))
}
