use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{Json, extract::State, response::IntoResponse, http::header};
use axum_extra::extract::cookie::{Cookie, SameSite};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Serialize, Deserialize};
use tracing::instrument;
use utoipa::ToSchema;

use crate::{
    infra::repositories,
    server::AppState,
};
use super::error::AuthError;

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LoginResponse {
    pub code: i32,
    pub data: Option<String>,
    pub msg: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TokenClaims {
    pub sub: i32,
    pub perms: Vec<i32>,
    pub iat: usize,
    pub exp: usize,
}

#[utoipa::path(
    post,
    path = "/login",
    request_body(
        content = LoginRequest,
        content_type = "application/x-www-form-urlencoded",
    ),
    responses(
        (status = 200, description = "Login successfully", body = LoginResponse),
    )
)]
#[instrument(skip(state))]
pub async fn login(
    State(state): State<AppState>,
    Json(user): Json<LoginRequest>,
) -> Result<impl IntoResponse, AuthError> {
    let user_in_db = repositories::user::try_get_by_username(
        &state.pg_pool, user.username.clone()
    )
        .await
        .map_err(AuthError::RepoError)?
        .ok_or(AuthError::InvalidCredentials)?;

    let is_valid = match PasswordHash::new(&user_in_db.hashed_password) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(user.password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true),
        Err(_) => false,
    };

    if !is_valid {
        return Err(AuthError::InvalidCredentials);
    }

    let perms = repositories::user::get_permissions(
        &state.pg_pool, user_in_db.id
    )
        .await
        .map_err(AuthError::RepoError)?
        .into_iter()
        .map(|perm| perm.id)
        .collect();

    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::minutes(60)).timestamp() as usize;
    let claims = TokenClaims {
        sub: user_in_db.id,
        perms,
        iat,
        exp,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.jwt_secret.as_ref()),
    )
        .map_err(|_| AuthError::InternalServerError("failed to encode the token".to_string()))?
        .to_owned();

    let cookie = Cookie::build(("token", token.clone()))
        .path("/")
        .max_age(time::Duration::days(7))
        .same_site(SameSite::Lax)
        .http_only(true);

    let mut response = Json(LoginResponse {
        code: 0,
        data: Some(token),
        msg: None,
    })
        .into_response();

    response
        .headers_mut()
        .insert(
            header::SET_COOKIE,
            cookie.to_string().parse().unwrap(),
        );

    Ok(response)
}
