use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use tracing::instrument;
use utoipa::ToSchema;

use crate::{
    infra::repositories::{self, ds_item::NewDatasetItemDB},
    server::AppState,
    utils::extractors::json::JsonExtractor,
};
use super::{error::DatasetItemError, schema::DatasetItemSchema};

#[derive(Debug, Deserialize, ToSchema)]
pub struct DatasetItemCreationRequest {
    pub typ: String,
    pub uri: String,
}

impl Into<NewDatasetItemDB> for DatasetItemCreationRequest {
    fn into(self) -> NewDatasetItemDB {
        NewDatasetItemDB {
            typ: self.typ,
            uri: self.uri,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DatasetItemCreationResponse {
    pub code: i32,
    pub data: Option<DatasetItemSchema>,
    pub msg: Option<String>,
}

#[utoipa::path(
    post,
    path = "/v1/datasets/items",
    request_body = DatasetItemCreationRequest,
    responses(
        (
            status = 200,
            description = "Dataset item created successfully",
            body = DatasetItemCreationResponse,
        ),
    )
)]
#[instrument(skip(state))]
pub async fn create_dataset_item(
    State(state): State<AppState>,
    JsonExtractor(new_item): JsonExtractor<DatasetItemCreationRequest>,
) -> Result<Json<DatasetItemCreationResponse>, DatasetItemError> {
    let item_in_db = repositories::ds_item::try_get_by_uri(
        &state.pg_pool, new_item.uri.clone()
    )
        .await
        .map_err(DatasetItemError::RepoError)?;

    if item_in_db.is_some() {
        return Err(DatasetItemError::Duplicate);
    }

    let created_item = repositories::ds_item::create(
        &state.pg_pool, new_item.into()
    )
        .await
        .map_err(DatasetItemError::RepoError)?;

    Ok(Json(DatasetItemCreationResponse {
        code: 0,
        data: Some(DatasetItemSchema::from(created_item)),
        msg: None,
    }))
}
