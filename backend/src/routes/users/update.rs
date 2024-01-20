use axum::{extract::State, Json};
use password_auth::generate_hash;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use utoipa::ToSchema;

use crate::{
    infra::repositories::{self, user::UpdatedUserDB},
    server::AppState,
    utils::extractors::{
        json::JsonExtractor,
        path::PathExtractor,
    },
};
use super::{error::UserError, schema::UserSchema};

#[derive(Debug, Deserialize, ToSchema)]
pub struct UserUpdateRequest {
    pub password: Option<String>,
    pub nickname: Option<String>,
    pub avatar_uri: Option<String>,
}

impl Into<UpdatedUserDB> for UserUpdateRequest {
    fn into(self) -> UpdatedUserDB {
        UpdatedUserDB {
            hashed_password: self.password
                .map(|password| generate_hash(password.as_bytes())),
            nickname: self.nickname,
            avatar_uri: self.avatar_uri,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserUpdateResponse {
    pub code: i32,
    pub data: Option<UserSchema>,
    pub msg: Option<String>,
}

#[utoipa::path(
    put,
    path = "/v1/users/{id}",
    request_body = UserUpdateRequest,
    responses(
        (
            status = 200,
            description = "User update successfully",
            body = UserUpdateResponse,
        ),
    )
)]
#[instrument(skip(state))]
pub async fn update_user(
    State(state): State<AppState>,
    PathExtractor(user_id): PathExtractor<i32>,
    JsonExtractor(updated_user): JsonExtractor<UserUpdateRequest>,
) -> Result<Json<UserUpdateResponse>, UserError> {
    repositories::user::try_get_by_id(
        &state.pg_pool, user_id
    )
        .await
        .map_err(UserError::RepoError)?
        .ok_or(UserError::NotFound)?;

    let user = repositories::user::update_by_id(
        &state.pg_pool, user_id, updated_user.into()
    )
        .await
        .map_err(UserError::RepoError)?;

    Ok(Json(UserUpdateResponse {
        code: 0,
        data: Some(UserSchema::from(user)),
        msg: None,
    }))
}
