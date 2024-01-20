use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use tracing::instrument;
use utoipa::ToSchema;

use crate::{
    infra::repositories::{self, permission::NewPermissionDB},
    server::AppState,
    utils::extractors::json::JsonExtractor,
};
use super::{error::PermissionError, schema::PermissionSchema};

#[derive(Debug, Deserialize, ToSchema)]
pub struct PermissionCreationRequest {
    pub name: String,
}

impl Into<NewPermissionDB> for PermissionCreationRequest {
    fn into(self) -> NewPermissionDB {
        NewPermissionDB {
            name: self.name,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PermissionCreationResponse {
    pub code: i32,
    pub data: Option<PermissionSchema>,
    pub msg: Option<String>,
}

#[utoipa::path(
    post,
    path = "/v1/permissions",
    request_body = PermissionCreationRequest,
    responses(
        (
            status = 200,
            description = "Permission created successfully",
            body = PermissionCreationResponse,
        ),
    )
)]
#[instrument(skip(state))]
pub async fn create_permission(
    State(state): State<AppState>,
    JsonExtractor(new_perm): JsonExtractor<PermissionCreationRequest>,
) -> Result<Json<PermissionCreationResponse>, PermissionError> {
    let perm_in_db = repositories::permission::try_get_by_name(
        &state.pg_pool, new_perm.name.clone()
    )
        .await
        .map_err(PermissionError::RepoError)?;

    if perm_in_db.is_some() {
        return Err(PermissionError::Duplicate);
    }

    let created_perm = repositories::permission::create(
        &state.pg_pool, new_perm.into()
    )
        .await
        .map_err(PermissionError::RepoError)?;

    Ok(Json(PermissionCreationResponse {
        code: 0,
        data: Some(PermissionSchema::from(created_perm)),
        msg: None,
    }))
}
