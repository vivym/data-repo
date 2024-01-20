use axum::Json;
use serde::Serialize;
use tracing::instrument;
use utoipa::ToSchema;

use crate::services::auth::AuthSession;
use super::error::AuthError;

#[derive(Debug, Serialize, ToSchema)]
pub struct LogoutResponse {
    pub code: i32,
    pub msg: Option<String>,
}

#[utoipa::path(
    get,
    path = "/v1/logout",
    responses(
        (status = 200, description = "Logout successfully", body = LogoutResponse),
    )
)]
#[instrument(skip(auth_session))]
pub async fn logout(mut auth_session: AuthSession) -> Result<Json<LogoutResponse>, AuthError> {
    match auth_session.logout().await {
        Ok(_) => Ok(Json(LogoutResponse {
            code: 0,
            msg: None,
        })),
        Err(_) => Err(AuthError::InternalServerError("logout failed".to_owned())),
    }
}
