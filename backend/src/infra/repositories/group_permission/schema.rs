use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::domain::models::group_perm::GroupPermModel;
use crate::infra::db::schema::groups_permissions;

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(primary_key(group_id, permission_id))]
#[diesel(table_name = groups_permissions)]      // Use the 'groups_permissions' table
#[diesel(check_for_backend(diesel::pg::Pg))]    // Check compatibility with PostgreSQL
pub struct GroupPermDB {
    pub group_id: i32,
    pub permission_id: i32,
    pub created_at: NaiveDateTime,
}

impl Into<GroupPermModel> for GroupPermDB {
    fn into(self) -> GroupPermModel {
        GroupPermModel {
            group_id: self.group_id,
            permission_id: self.permission_id,
            created_at: self.created_at,
        }
    }
}
