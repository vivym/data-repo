use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::domain::models::group::GroupModel;
use crate::infra::db::schema::groups;

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = groups)]                  // Use the 'groups' table
#[diesel(check_for_backend(diesel::pg::Pg))]    // Check compatibility with PostgreSQL
pub struct GroupDB {
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Into<GroupModel> for GroupDB {
    fn into(self) -> GroupModel {
        GroupModel {
            id: self.id,
            name: self.name,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}