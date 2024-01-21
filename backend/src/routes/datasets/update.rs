use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use tracing::instrument;
use utoipa::ToSchema;

use crate::{
    infra::repositories::{self, dataset::UpdatedDatasetDB},
    server::AppState,
    utils::extractors::{
        json::JsonExtractor,
        path::PathExtractor,
    },
};
use super::{error::DatasetError, schema::DatasetSchema};

#[derive(Debug, Deserialize, ToSchema)]
pub struct DatasetUpdateRequest {
    pub description: Option<String>,
}

impl Into<UpdatedDatasetDB> for DatasetUpdateRequest {
    fn into(self) -> UpdatedDatasetDB {
        UpdatedDatasetDB {
            description: self.description,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DatasetUpdateResponse {
    pub code: i32,
    pub data: Option<DatasetSchema>,
    pub msg: Option<String>,
}

#[utoipa::path(
    put,
    path = "/v1/datasets/{id}",
    request_body = DatasetUpdateRequest,
    responses(
        (
            status = 200,
            description = "Dataset update successfully",
            body = DatasetUpdateResponse,
        ),
    )
)]
#[instrument(skip(state))]
pub async fn update_dataset(
    State(state): State<AppState>,
    PathExtractor(ds_id): PathExtractor<i32>,
    JsonExtractor(updated_ds): JsonExtractor<DatasetUpdateRequest>,
) -> Result<Json<DatasetUpdateResponse>, DatasetError> {
    repositories::dataset::try_get_by_id(
        &state.pg_pool, ds_id
    )
        .await
        .map_err(DatasetError::RepoError)?
        .ok_or(DatasetError::NotFound)?;

    let dataset = repositories::dataset::update_by_id(
        &state.pg_pool, ds_id, updated_ds.into()
    )
        .await
        .map_err(DatasetError::RepoError)?;

    Ok(Json(DatasetUpdateResponse {
        code: 0,
        data: Some(DatasetSchema::from(dataset)),
        msg: None,
    }))
}
