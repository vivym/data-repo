use axum::{extract::{State, Query}, Json};
use serde::{Deserialize, Serialize};
use tracing::instrument;
use utoipa::{ToSchema, IntoParams};

use crate::{
    infra::repositories::{self, user::UsersFilter},
    server::AppState,
};
use super::{error::UserError, schema::UserSchema};

#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct UserSearchQuery {
    /// Username
    pub username: Option<String>,
    /// Nickname
    pub nickname: Option<String>,
    /// Email
    pub email: Option<String>,
    /// Verified
    pub verified: Option<bool>,
    /// Skip, default: 0
    pub skip: Option<i64>,
    /// Limit, default: 20
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ListUsersResponse {
    code: i32,
    data: Option<Vec<UserSchema>>,
    msg: Option<String>,
}

#[utoipa::path(
    get,
    path = "/v1/users",
    params(UserSearchQuery),
    responses(
        (status = 200, description = "User query successfully", body = ListUsersResponse),
        (status = NOT_FOUND, description = "User not found"),
    )
)]
#[instrument(skip(state))]
pub async fn list_users(
    State(state): State<AppState>,
    Query(params): Query<UsersFilter>,
) -> Result<Json<ListUsersResponse>, UserError> {
    let users = repositories::user::get_all(
        &state.pg_pool, params
    )
        .await
        .map_err(UserError::RepoError)?;

    let users = users
        .into_iter()
        .map(UserSchema::from)
        .collect();

    Ok(Json(ListUsersResponse {
        code: 0,
        data: Some(users),
        msg: None,
    }))
}
