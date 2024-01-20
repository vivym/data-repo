use axum::{extract::State, Json};
use password_auth::generate_hash;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use utoipa::ToSchema;

use crate::{
    infra::repositories::{self, user::NewUserDB},
    server::AppState,
    utils::extractors::json::JsonExtractor,
};
use super::{error::UserError, schema::UserSchema};

#[derive(Debug, Deserialize, ToSchema)]
pub struct UserCreationRequest {
    pub username: String,
    pub password: String,
    pub nickname: String,
    pub email: String,
    pub avatar_uri: String,
}

impl Into<NewUserDB> for UserCreationRequest {
    fn into(self) -> NewUserDB {
        NewUserDB {
            username: self.username,
            hashed_password: generate_hash(self.password.as_bytes()),
            nickname: self.nickname,
            avatar_uri: self.avatar_uri,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserCreationResponse {
    pub code: i32,
    pub data: Option<UserSchema>,
    pub msg: Option<String>,
}

#[utoipa::path(
    post,
    path = "/v1/users",
    request_body = UserCreationRequest,
    responses(
        (
            status = 200,
            description = "User created successfully",
            body = UserCreationResponse,
        ),
    )
)]
#[instrument(skip(state))]
pub async fn create_user(
    State(state): State<AppState>,
    JsonExtractor(new_user): JsonExtractor<UserCreationRequest>,
) -> Result<Json<UserCreationResponse>, UserError> {
    let user_in_db = repositories::user::try_get_by_username(
        &state.pg_pool, new_user.username.clone()
    )
        .await
        .map_err(UserError::RepoError)?;

    if user_in_db.is_some() {
        return Err(UserError::DuplicateUsername);
    }

    let created_user = repositories::user::create(
        &state.pg_pool, new_user.into()
    )
        .await
        .map_err(UserError::RepoError)?;

    Ok(Json(UserCreationResponse {
        code: 0,
        data: Some(UserSchema::from(created_user)),
        msg: None,
    }))
}
