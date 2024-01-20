use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::domain::models::permission::PermissionModel;
use crate::infra::db::schema::permissions;

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = permissions)]             // Use the 'permissions' table
#[diesel(check_for_backend(diesel::pg::Pg))]    // Check compatibility with PostgreSQL
pub struct PermissionDB {
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Into<PermissionModel> for PermissionDB {
    fn into(self) -> PermissionModel {
        PermissionModel {
            id: self.id,
            name: self.name,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
