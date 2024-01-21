use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use axum::{extract::State, Json};
use rand_core::OsRng;
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
    pub avatar_uri: String,
}

impl TryInto<NewUserDB> for UserCreationRequest {
    type Error = UserError;

    fn try_into(self) -> Result<NewUserDB, UserError> {
        let salt = SaltString::generate(&mut OsRng);
        let hashed_password = Argon2::default()
            .hash_password(self.password.as_bytes(), &salt)
            .map_err(|_| UserError::InternalServerError("failed to hash the password".to_owned()))?
            .to_string();

        Ok(NewUserDB {
            username: self.username,
            hashed_password,
            nickname: self.nickname,
            avatar_uri: self.avatar_uri,
        })
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
        &state.pg_pool, new_user.try_into()?
    )
        .await
        .map_err(UserError::RepoError)?;

    Ok(Json(UserCreationResponse {
        code: 0,
        data: Some(UserSchema::from(created_user)),
        msg: None,
    }))
}
