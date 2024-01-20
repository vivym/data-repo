use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use tracing::instrument;
use utoipa::ToSchema;

use crate::{
    infra::repositories,
    server::AppState,
    utils::extractors::path::PathExtractor,
};
use super::{error::GroupError, schema::GroupSchema};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GetGroupResponse {
    pub code: i32,
    pub data: Option<GroupSchema>,
    pub msg: Option<String>,
}

#[utoipa::path(
    get,
    path = "/v1/groups/{id}",
    params(
        ("id", Path, description = "Group id")
    ),
    responses(
        (status = 200, description = "Group query successfully", body = GetGroupResponse),
        (status = NOT_FOUND, description = "Group not found"),
    )
)]
#[instrument(skip(state))]
pub async fn get_group(
    State(state): State<AppState>,
    PathExtractor(group_id): PathExtractor<i32>,
) -> Result<Json<GetGroupResponse>, GroupError> {
    let group = repositories::group::get_by_id(
        &state.pg_pool, group_id
    )
        .await
        .map_err(GroupError::RepoError)?;

    Ok(Json(GetGroupResponse {
        code: 0,
        data: Some(GroupSchema::from(group)),
        msg: None,
    }))
}
