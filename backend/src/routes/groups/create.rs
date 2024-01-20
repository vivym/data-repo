use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use tracing::instrument;
use utoipa::ToSchema;

use crate::{
    infra::repositories::{self, group::NewGroupDB},
    server::AppState,
    utils::extractors::json::JsonExtractor,
};
use super::{error::GroupError, schema::GroupSchema};

#[derive(Debug, Deserialize, ToSchema)]
pub struct GroupCreationRequest {
    pub name: String,
}

impl Into<NewGroupDB> for GroupCreationRequest {
    fn into(self) -> NewGroupDB {
        NewGroupDB {
            name: self.name,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GroupCreationResponse {
    pub code: i32,
    pub data: Option<GroupSchema>,
    pub msg: Option<String>,
}

#[utoipa::path(
    post,
    path = "/v1/groups",
    request_body = GroupCreationRequest,
    responses(
        (
            status = 200,
            description = "Group created successfully",
            body = GroupCreationResponse,
        ),
    )
)]
#[instrument(skip(state))]
pub async fn create_group(
    State(state): State<AppState>,
    JsonExtractor(new_group): JsonExtractor<GroupCreationRequest>,
) -> Result<Json<GroupCreationResponse>, GroupError> {
    let group_in_db = repositories::group::try_get_by_name(
        &state.pg_pool, new_group.name.clone()
    )
        .await
        .map_err(GroupError::RepoError)?;

    if group_in_db.is_some() {
        return Err(GroupError::Duplicate);
    }

    let created_group = repositories::group::create(
        &state.pg_pool, new_group.into()
    )
        .await
        .map_err(GroupError::RepoError)?;

    Ok(Json(GroupCreationResponse {
        code: 0,
        data: Some(GroupSchema::from(created_group)),
        msg: None,
    }))
}
