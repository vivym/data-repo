use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use tracing::instrument;
use utoipa::ToSchema;

use crate::{
    infra::repositories::{self, ds_item::UpdatedDatasetItemDB},
    server::AppState,
    utils::extractors::{
        json::JsonExtractor,
        path::PathExtractor,
    },
};
use super::{error::DatasetItemError, schema::DatasetItemSchema};

#[derive(Debug, Deserialize, ToSchema)]
pub struct DatasetItemUpdateRequest {
    pub typ: Option<String>,
    pub uri: Option<String>,
}

impl Into<UpdatedDatasetItemDB> for DatasetItemUpdateRequest {
    fn into(self) -> UpdatedDatasetItemDB {
        UpdatedDatasetItemDB {
            typ: self.typ,
            uri: self.uri,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DatasetItemUpdateResponse {
    pub code: i32,
    pub data: Option<DatasetItemSchema>,
    pub msg: Option<String>,
}

#[utoipa::path(
    put,
    path = "/v1/datasets/items/{id}",
    request_body = DatasetItemUpdateRequest,
    responses(
        (
            status = 200,
            description = "Dataset items update successfully",
            body = DatasetItemUpdateResponse,
        ),
    )
)]
#[instrument(skip(state))]
pub async fn update_dataset_item(
    State(state): State<AppState>,
    PathExtractor(item_id): PathExtractor<i32>,
    JsonExtractor(updated_item): JsonExtractor<DatasetItemUpdateRequest>,
) -> Result<Json<DatasetItemUpdateResponse>, DatasetItemError> {
    repositories::ds_item::try_get_by_id(
        &state.pg_pool, item_id
    )
        .await
        .map_err(DatasetItemError::RepoError)?
        .ok_or(DatasetItemError::NotFound)?;

    let dataset = repositories::ds_item::update_by_id(
        &state.pg_pool, item_id, updated_item.into()
    )
        .await
        .map_err(DatasetItemError::RepoError)?;

    Ok(Json(DatasetItemUpdateResponse {
        code: 0,
        data: Some(DatasetItemSchema::from(dataset)),
        msg: None,
    }))
}
