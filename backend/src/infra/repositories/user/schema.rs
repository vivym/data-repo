use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::domain::models::user::UserModel;
use crate::infra::db::schema::users;

#[derive(Queryable, Selectable)]
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
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
