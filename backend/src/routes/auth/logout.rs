use axum::{Json, response::IntoResponse, http::header};
use axum_extra::extract::cookie::{Cookie, SameSite};
use serde::Serialize;
use tracing::instrument;
use utoipa::ToSchema;

use super::error::AuthError;

#[derive(Debug, Serialize, ToSchema)]
pub struct LogoutResponse {
    pub code: i32,
    pub msg: Option<String>,
}

#[utoipa::path(
    get,
    path = "/logout",
    responses(
        (status = 200, description = "Logout successfully", body = LogoutResponse),
    )
)]
#[instrument]
pub async fn logout() -> Result<impl IntoResponse, AuthError> {
    let cookie = Cookie::build(("token", ""))
        .path("/")
        .max_age(time::Duration::hours(-1))
        .same_site(SameSite::Lax)
        .http_only(true);

    let mut response = Json(LogoutResponse {
        code: 0,
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
