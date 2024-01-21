use axum::{extract::State, Json, Extension};
use serde::{Deserialize, Serialize};
use tracing::instrument;
use utoipa::ToSchema;

use crate::{
    domain::models::user::UserModel,
    infra::repositories,
    routes::groups::schema::GroupSchema,
    server::AppState,
    utils::extractors::path::PathExtractor,
};
use super::error::UserError;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GetUserGroupsResponse {
    pub code: i32,
    pub data: Option<Vec<GroupSchema>>,
    pub msg: Option<String>,
}

#[utoipa::path(
    get,
    path = "/v1/users/{id}/groups",
    params(
        ("id", Path, description = "User id")
    ),
    responses(
        (status = 200, description = "User query successfully", body = GetUserGroupsResponse),
        (status = NOT_FOUND, description = "User not found"),
    )
)]
#[instrument(skip(state))]
pub async fn get_user_groups(
    State(state): State<AppState>,
    PathExtractor(user_id): PathExtractor<i32>,
) -> Result<Json<GetUserGroupsResponse>, UserError> {
    let groups = repositories::user::get_groups(
        &state.pg_pool, user_id
    )
        .await
        .map_err(UserError::RepoError)?;

    Ok(Json(GetUserGroupsResponse {
        code: 0,
        data: Some(groups.into_iter().map(Into::into).collect()),
        msg: None,
    }))
}

#[utoipa::path(
    get,
    path = "/v1/users/me/groups",
    responses(
        (status = 200, description = "User query successfully", body = GetUserGroupsResponse),
        (status = NOT_FOUND, description = "User not found"),
    )
)]
#[instrument(skip(state))]
pub async fn get_me_groups(
    State(state): State<AppState>,
    Extension(user): Extension<UserModel>,
) -> Result<Json<GetUserGroupsResponse>, UserError> {
    let groups = repositories::user::get_groups(
        &state.pg_pool, user.id
    )
        .await
        .map_err(UserError::RepoError)?;

    Ok(Json(GetUserGroupsResponse {
        code: 0,
        data: Some(groups.into_iter().map(Into::into).collect()),
        msg: None,
    }))
}
