use axum::{extract::{State, Query}, Json};
use serde::{Deserialize, Serialize};
use tracing::instrument;
use utoipa::{ToSchema, IntoParams};

use crate::{
    infra::repositories::{self, group::GroupsFilter},
    server::AppState,
};
use super::{error::GroupError, schema::GroupSchema};

#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct GroupSearchQuery {
    /// Skip, default: 0
    pub skip: Option<i64>,
    /// Limit, default: 20
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ListGroupsResponse {
    code: i32,
    data: Option<Vec<GroupSchema>>,
    msg: Option<String>,
}

#[utoipa::path(
    get,
    path = "/v1/groups",
    params(GroupSearchQuery),
    responses(
        (status = 200, description = "Group query successfully", body = ListGroupsResponse),
        (status = NOT_FOUND, description = "Group not found"),
    )
)]
#[instrument(skip(state))]
pub async fn list_groups(
    State(state): State<AppState>,
    Query(params): Query<GroupsFilter>,
) -> Result<Json<ListGroupsResponse>, GroupError> {
    let groups = repositories::group::get_all(
        &state.pg_pool, params
    )
        .await
        .map_err(GroupError::RepoError)?;

    let groups = groups
        .into_iter()
        .map(GroupSchema::from)
        .collect();

    Ok(Json(ListGroupsResponse {
        code: 0,
        data: Some(groups),
        msg: None,
    }))
}
