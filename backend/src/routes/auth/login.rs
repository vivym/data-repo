use axum::{Form, Json};
use serde::{Serialize, Deserialize};
use tracing::instrument;
use utoipa::ToSchema;

use crate::services::auth::{AuthSession, Credentials};
use super::error::AuthError;

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LoginResponse {
    pub code: i32,
    pub msg: Option<String>,
}

#[utoipa::path(
    post,
    path = "/v1/login",
    request_body(
        content = LoginRequest,
        content_type = "application/x-www-form-urlencoded",
    ),
    responses(
        (status = 200, description = "Login successfully", body = LoginResponse),
    )
)]
#[instrument(skip(auth_session))]
pub async fn login(
    mut auth_session: AuthSession,
    Form(creds): Form<Credentials>,
) -> Result<Json<LoginResponse>, AuthError> {
    let user = match auth_session.authenticate(creds.clone()).await {
        Ok(Some(user)) => user,
        Ok(None) => return Err(AuthError::InvalidCredentials),
        Err(_) => return Err(AuthError::InternalServerError("authenticate failed".to_owned())),
    };

    if auth_session.login(&user).await.is_err() {
        return Err(AuthError::InternalServerError("login failed".to_owned()));
    }

    Ok(Json(LoginResponse {
        code: 0,
        msg: None,
    }))
}
