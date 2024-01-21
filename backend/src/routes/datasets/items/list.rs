use axum::{extract::{State, Query}, Json};
use serde::{Deserialize, Serialize};
use tracing::instrument;
use utoipa::{ToSchema, IntoParams};

use crate::{
    infra::repositories::{self, ds_item::DatasetItemsFilter},
    server::AppState,
};
use super::{error::DatasetItemError, schema::DatasetItemSchema};

#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct DatasetItemSearchQuery {
    /// Dataset ID
    pub ds_id: Option<i32>,
    /// Skip, default: 0
    pub skip: Option<i64>,
    /// Limit, default: 20
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ListDatasetItemsResponse {
    code: i32,
    data: Option<Vec<DatasetItemSchema>>,
    msg: Option<String>,
}

#[utoipa::path(
    get,
    path = "/v1/datasets/items",
    params(DatasetItemSearchQuery),
    responses(
        (
            status = 200,
            description = "Dataset item query successfully",
            body = ListDatasetItemsResponse,
        ),
        (status = NOT_FOUND, description = "Dataset item not found"),
    )
)]
#[instrument(skip(state))]
pub async fn list_dataset_items(
    State(state): State<AppState>,
    Query(params): Query<DatasetItemsFilter>,
) -> Result<Json<ListDatasetItemsResponse>, DatasetItemError> {
    let items = repositories::ds_item::get_all(
        &state.pg_pool, params
    )
        .await
        .map_err(DatasetItemError::RepoError)?;

    let items = items
        .into_iter()
        .map(DatasetItemSchema::from)
        .collect();

    Ok(Json(ListDatasetItemsResponse {
        code: 0,
        data: Some(items),
        msg: None,
    }))
}
