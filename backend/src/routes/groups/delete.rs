use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use tracing::instrument;
use utoipa::ToSchema;

use crate::{
    infra::repositories,
    server::AppState,
    utils::extractors::path::PathExtractor,
};
use super::error::GroupError;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeleteGroupResponse {
    pub code: i32,
    pub data: bool,
    pub msg: Option<String>,
}

#[utoipa::path(
    delete,
    path = "/v1/groups/{id}",
    params(
        ("id", Path, description = "Group id")
    ),
    responses(
        (status = 200, description = "Group deletion successfully", body = DeleteGroupResponse),
        (status = NOT_FOUND, description = "Group not found"),
    )
)]
#[instrument(skip(state))]
pub async fn delete_group(
    State(state): State<AppState>,
    PathExtractor(group_id): PathExtractor<i32>,
) -> Result<Json<DeleteGroupResponse>, GroupError> {
    repositories::group::delete_by_id(&state.pg_pool, group_id)
        .await
        .map_err(GroupError::RepoError)?;

    Ok(Json(DeleteGroupResponse {
        code: 0,
        data: true,
        msg: None,
    }))
}
