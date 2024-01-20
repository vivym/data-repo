use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::domain::models::user::UserModel;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserSchema {
    pub id: i32,
    pub username: String,
    pub nickname: String,
    pub avatar_uri: String,
    pub verified: bool,
    #[schema(value_type = String)]
    created_at: NaiveDateTime,
    #[schema(value_type = String)]
    updated_at: NaiveDateTime,
}

impl From<UserModel> for UserSchema {
    fn from(user: UserModel) -> Self {
        Self {
            id: user.id,
            username: user.username,
            nickname: user.nickname,
            avatar_uri: user.avatar_uri,
            verified: user.verified,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}
