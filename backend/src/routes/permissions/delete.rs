use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use tracing::instrument;
use utoipa::ToSchema;

use crate::{
    infra::repositories,
    server::AppState,
    utils::extractors::path::PathExtractor,
};
use super::error::PermissionError;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeletePermissionResponse {
    pub code: i32,
    pub data: bool,
    pub msg: Option<String>,
}

#[utoipa::path(
    delete,
    path = "/v1/permission/{id}",
    params(
        ("id", Path, description = "Permission id")
    ),
    responses(
        (status = 200, description = "Permission deletion successfully", body = DeletePermissionResponse),
        (status = NOT_FOUND, description = "Permission not found"),
    )
)]
#[instrument(skip(state))]
pub async fn delete_permission(
    State(state): State<AppState>,
    PathExtractor(perm_id): PathExtractor<i32>,
) -> Result<Json<DeletePermissionResponse>, PermissionError> {
    repositories::permission::delete_by_id(&state.pg_pool, perm_id)
        .await
        .map_err(PermissionError::RepoError)?;

    Ok(Json(DeletePermissionResponse {
        code: 0,
        data: true,
        msg: None,
    }))
}
