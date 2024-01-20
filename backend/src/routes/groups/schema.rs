use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::domain::models::group::GroupModel;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GroupSchema {
    pub id: i32,
    pub name: String,
    #[schema(value_type = String)]
    created_at: NaiveDateTime,
    #[schema(value_type = String)]
    updated_at: NaiveDateTime,
}

impl From<GroupModel> for GroupSchema {
    fn from(user: GroupModel) -> Self {
        Self {
            id: user.id,
            name: user.name,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}
