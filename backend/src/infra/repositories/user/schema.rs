use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::domain::models::user::UserModel;
use crate::infra::db::schema::users;
use crate::infra::repositories::group::GroupDB;

#[derive(Queryable, Selectable, Identifiable, Clone)]
#[diesel(table_name = users)]                   // Use the 'users' table
#[diesel(check_for_backend(diesel::pg::Pg))]    // Check compatibility with PostgreSQL
pub struct UserDB {
    pub id: i32,
    pub username: String,
    pub hashed_password: String,
    pub nickname: String,
    pub avatar_uri: String,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Into<UserModel> for UserDB {
    fn into(self) -> UserModel {
        UserModel {
            id: self.id,
            username: self.username,
            hashed_password: self.hashed_password,
            nickname: self.nickname,
            avatar_uri: self.avatar_uri,
            is_active: self.is_active,
            groups: None,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

impl Into<UserModel> for (UserDB, Vec<GroupDB>) {
    fn into(self) -> UserModel {
        UserModel {
            id: self.0.id,
            username: self.0.username,
            hashed_password: self.0.hashed_password,
            nickname: self.0.nickname,
            avatar_uri: self.0.avatar_uri,
            is_active: self.0.is_active,
            groups: Some(self.1.into_iter().map(|g| g.into()).collect()),
            created_at: self.0.created_at,
            updated_at: self.0.updated_at,
        }
    }
}
