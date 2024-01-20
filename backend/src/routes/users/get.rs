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
pub struct GetUserResponse {
    pub code: i32,
    pub data: Option<UserSchema>,
    pub msg: Option<String>,
}

#[utoipa::path(
    get,
    path = "/v1/users/{id}",
    params(
        ("id", Path, description = "User id")
    ),
    responses(
        (status = 200, description = "User query successfully", body = GetUserResponse),
        (status = NOT_FOUND, description = "User not found"),
    )
)]
#[instrument(skip(state))]
pub async fn get_user(
    State(state): State<AppState>,
    PathExtractor(user_id): PathExtractor<i32>,
) -> Result<Json<GetUserResponse>, UserError> {
    let user = repositories::user::get_by_id(
        &state.pg_pool, user_id
    )
        .await
        .map_err(UserError::RepoError)?;

    Ok(Json(GetUserResponse {
        code: 0,
        data: Some(UserSchema::from(user)),
        msg: None,
    }))
}
