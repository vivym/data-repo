use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use tracing::instrument;
use utoipa::ToSchema;

use crate::{
    infra::repositories,
    server::AppState,
    utils::extractors::path::PathExtractor,
};
use super::error::UserError;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeleteUserResponse {
    pub code: i32,
    pub data: bool,
    pub msg: Option<String>,
}

#[utoipa::path(
    delete,
    path = "/v1/users/{id}",
    params(
        ("id", Path, description = "User id")
    ),
    responses(
        (status = 200, description = "User deletion successfully", body = DeleteUserResponse),
        (status = NOT_FOUND, description = "User not found"),
    )
)]
#[instrument(skip(state))]
pub async fn delete_user(
    State(state): State<AppState>,
    PathExtractor(user_id): PathExtractor<i32>,
) -> Result<Json<DeleteUserResponse>, UserError> {
    repositories::user::delete_by_id(&state.pg_pool, user_id)
        .await
        .map_err(UserError::RepoError)?;

    Ok(Json(DeleteUserResponse {
        code: 0,
        data: true,
        msg: None,
    }))
}
