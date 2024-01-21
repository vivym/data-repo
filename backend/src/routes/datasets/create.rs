use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use tracing::instrument;
use utoipa::ToSchema;

use crate::{
    infra::repositories::{self, dataset::NewDatasetDB},
    server::AppState,
    utils::extractors::json::JsonExtractor,
};
use super::{error::DatasetError, schema::DatasetSchema};

#[derive(Debug, Deserialize, ToSchema)]
pub struct DatasetCreationRequest {
    pub name: String,
    pub description: String,
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
pub struct DatasetCreationResponse {
    pub code: i32,
    pub data: Option<DatasetSchema>,
    pub msg: Option<String>,
}

#[utoipa::path(
    post,
    path = "/v1/datasets",
    request_body = DatasetCreationRequest,
    responses(
        (
            status = 200,
            description = "Dataset created successfully",
            body = DatasetCreationResponse,
        ),
    )
)]
#[instrument(skip(state))]
pub async fn create_dataset(
    State(state): State<AppState>,
    JsonExtractor(new_ds): JsonExtractor<DatasetCreationRequest>,
) -> Result<Json<DatasetCreationResponse>, DatasetError> {
    let ds_in_db = repositories::dataset::try_get_by_name(
        &state.pg_pool, new_ds.name.clone()
    )
        .await
        .map_err(DatasetError::RepoError)?;

    if ds_in_db.is_some() {
        return Err(DatasetError::Duplicate);
    }

    let created_ds = repositories::dataset::create(
        &state.pg_pool, new_ds.into()
    )
        .await
        .map_err(DatasetError::RepoError)?;

    Ok(Json(DatasetCreationResponse {
        code: 0,
        data: Some(DatasetSchema::from(created_ds)),
        msg: None,
    }))
}
