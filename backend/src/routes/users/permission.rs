use axum::{extract::State, Json, Extension};
use serde::{Deserialize, Serialize};
use tracing::instrument;
use utoipa::ToSchema;

use crate::{
    domain::models::user::UserModel,
    infra::repositories,
    routes::permissions::schema::PermissionSchema,
    server::AppState,
    utils::extractors::path::PathExtractor,
};
use super::error::UserError;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GetUserPermissionsResponse {
    pub code: i32,
    pub data: Option<Vec<PermissionSchema>>,
    pub msg: Option<String>,
}

#[utoipa::path(
    get,
    path = "/v1/users/{id}/permissions",
    params(
        ("id", Path, description = "User id")
    ),
    responses(
        (status = 200, description = "User query successfully", body = GetUserPermissionsResponse),
        (status = NOT_FOUND, description = "User not found"),
    )
)]
#[instrument(skip(state))]
pub async fn get_user_permissions(
    State(state): State<AppState>,
    PathExtractor(user_id): PathExtractor<i32>,
) -> Result<Json<GetUserPermissionsResponse>, UserError> {
    let perms = repositories::user::get_permissions(
        &state.pg_pool, user_id
    )
        .await
        .map_err(UserError::RepoError)?;

    Ok(Json(GetUserPermissionsResponse {
        code: 0,
        data: Some(perms.into_iter().map(Into::into).collect()),
        msg: None,
    }))
}

#[utoipa::path(
    get,
    path = "/v1/users/me/permissions",
    responses(
        (status = 200, description = "User query successfully", body = GetUserPermissionsResponse),
        (status = NOT_FOUND, description = "User not found"),
    )
)]
#[instrument(skip(state))]
pub async fn get_me_permissions(
    State(state): State<AppState>,
    Extension(user): Extension<UserModel>,
) -> Result<Json<GetUserPermissionsResponse>, UserError> {
    let perms = repositories::user::get_permissions(
        &state.pg_pool, user.id
    )
        .await
        .map_err(UserError::RepoError)?;

    Ok(Json(GetUserPermissionsResponse {
        code: 0,
        data: Some(perms.into_iter().map(Into::into).collect()),
        msg: None,
    }))
}
