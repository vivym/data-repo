use axum::{extract::{State, Query}, Json};
use serde::{Deserialize, Serialize};
use tracing::instrument;
use utoipa::{ToSchema, IntoParams};

use crate::{
    infra::repositories::{self, permission::PermissionsFilter},
    server::AppState,
};
use super::{error::PermissionError, schema::PermissionSchema};

#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct PermissionSearchQuery {
    /// Skip, default: 0
    pub skip: Option<i64>,
    /// Limit, default: 20
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ListPermissionsResponse {
    code: i32,
    data: Option<Vec<PermissionSchema>>,
    msg: Option<String>,
}

#[utoipa::path(
    get,
    path = "/v1/permissions",
    params(PermissionSearchQuery),
    responses(
        (status = 200, description = "Permission query successfully", body = ListPermissionsResponse),
        (status = NOT_FOUND, description = "Permission not found"),
    )
)]
#[instrument(skip(state))]
pub async fn list_permissions(
    State(state): State<AppState>,
    Query(params): Query<PermissionsFilter>,
) -> Result<Json<ListPermissionsResponse>, PermissionError> {
    let users = repositories::permission::get_all(
        &state.pg_pool, params
    )
        .await
        .map_err(PermissionError::RepoError)?;

    let users = users
        .into_iter()
        .map(PermissionSchema::from)
        .collect();

    Ok(Json(ListPermissionsResponse {
        code: 0,
        data: Some(users),
        msg: None,
    }))
}
