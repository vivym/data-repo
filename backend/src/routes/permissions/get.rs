use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use tracing::instrument;
use utoipa::ToSchema;

use crate::{
    infra::repositories,
    server::AppState,
    utils::extractors::path::PathExtractor,
};
use super::{error::PermissionError, schema::PermissionSchema};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GetPermissionResponse {
    pub code: i32,
    pub data: Option<PermissionSchema>,
    pub msg: Option<String>,
}

#[utoipa::path(
    get,
    path = "/v1/permissions/{id}",
    params(
        ("id", Path, description = "Permission id")
    ),
    responses(
        (status = 200, description = "Permission query successfully", body = GetPermissionResponse),
        (status = NOT_FOUND, description = "Permission not found"),
    )
)]
#[instrument(skip(state))]
pub async fn get_permission(
    State(state): State<AppState>,
    PathExtractor(perm_id): PathExtractor<i32>,
) -> Result<Json<GetPermissionResponse>, PermissionError> {
    let user = repositories::permission::get_by_id(
        &state.pg_pool, perm_id
    )
        .await
        .map_err(PermissionError::RepoError)?;

    Ok(Json(GetPermissionResponse {
        code: 0,
        data: Some(PermissionSchema::from(user)),
        msg: None,
    }))
}
