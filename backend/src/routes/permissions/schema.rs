use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::domain::models::permission::PermissionModel;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PermissionSchema {
    pub id: i32,
    pub name: String,
    #[schema(value_type = String)]
    created_at: NaiveDateTime,
    #[schema(value_type = String)]
    updated_at: NaiveDateTime,
}

impl From<PermissionModel> for PermissionSchema {
    fn from(perm: PermissionModel) -> Self {
        Self {
            id: perm.id,
            name: perm.name,
            created_at: perm.created_at,
            updated_at: perm.updated_at,
        }
    }
}
