use axum::{extract::{State, Query}, Json};
use serde::{Deserialize, Serialize};
use tracing::instrument;
use utoipa::{ToSchema, IntoParams};

use crate::{
    infra::repositories::{self, dataset::DatasetsFilter},
    server::AppState,
};
use super::{error::DatasetError, schema::DatasetSchema};

#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct DatasetSearchQuery {
    /// Skip, default: 0
    pub skip: Option<i64>,
    /// Limit, default: 20
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ListDatasetsResponse {
    code: i32,
    data: Option<Vec<DatasetSchema>>,
    msg: Option<String>,
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
    let datasets = repositories::dataset::get_all(
        &state.pg_pool, params
    )
        .await
        .map_err(DatasetError::RepoError)?;

    let datasets = datasets
        .into_iter()
        .map(DatasetSchema::from)
        .collect();

    Ok(Json(ListDatasetsResponse {
        code: 0,
        data: Some(datasets),
        msg: None,
    }))
}
