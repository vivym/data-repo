use std::hash::Hash;

use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::domain::models::group::GroupModel;
use crate::domain::models::permission::PermissionModel;
use crate::infra::db::schema::groups;

#[derive(Queryable, Selectable, Identifiable, Clone, Eq)]
#[diesel(table_name = groups)]                  // Use the 'groups' table
#[diesel(check_for_backend(diesel::pg::Pg))]    // Check compatibility with PostgreSQL
pub struct GroupDB {
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl PartialEq for GroupDB {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Hash for GroupDB {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Into<GroupModel> for GroupDB {
    fn into(self) -> GroupModel {
        GroupModel {
            id: self.id,
            name: self.name,
            permissions: None,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

impl Into<GroupModel> for (GroupDB, Vec<PermissionModel>) {
    fn into(self) -> GroupModel {
        GroupModel {
            id: self.0.id,
            name: self.0.name,
            permissions: Some(self.1),
            created_at: self.0.created_at,
            updated_at: self.0.updated_at,
        }
    }
}
