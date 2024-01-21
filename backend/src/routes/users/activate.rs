use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use tracing::instrument;
use utoipa::ToSchema;

use crate::{
    infra::repositories,
    server::AppState,
    utils::extractors::path::PathExtractor,
};
use super::{error::UserError, schema::UserSchema};


#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ActivateUserResponse {
    pub code: i32,
    pub data: Option<UserSchema>,
    pub msg: Option<String>,
}

#[utoipa::path(
    get,
    path = "/v1/users/{id}/activate",
    params(
        ("id", Path, description = "User id")
    ),
    responses(
        (
            status = 200,
            description = "User activated",
            body = ActivateUserResponse,
        ),
    )
)]
#[instrument(skip(state))]
pub async fn activate_user(
    State(state): State<AppState>,
    PathExtractor(user_id): PathExtractor<i32>,
) -> Result<Json<ActivateUserResponse>, UserError> {
    repositories::user::try_get_by_id(
        &state.pg_pool, user_id
    )
        .await
        .map_err(UserError::RepoError)?
        .ok_or(UserError::NotFound)?;

    let user = repositories::user::activate_by_id(
        &state.pg_pool, user_id
    )
        .await
        .map_err(UserError::RepoError)?;

    Ok(Json(ActivateUserResponse {
        code: 0,
        data: Some(UserSchema::from(user)),
        msg: None,
    }))
}
