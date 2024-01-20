use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::domain::models::user_group::UserGroupModel;
use crate::infra::db::schema::users_groups;

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(primary_key(user_id, group_id))]
#[diesel(table_name = users_groups)]            // Use the 'users_groups' table
#[diesel(check_for_backend(diesel::pg::Pg))]    // Check compatibility with PostgreSQL
pub struct UserGroupDB {
    pub user_id: i32,
    pub group_id: i32,
    pub created_at: NaiveDateTime,
}

impl Into<UserGroupModel> for UserGroupDB {
    fn into(self) -> UserGroupModel {
        UserGroupModel {
            user_id: self.user_id,
            group_id: self.group_id,
            created_at: self.created_at,
        }
    }
}
