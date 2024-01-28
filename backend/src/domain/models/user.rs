use chrono::NaiveDateTime;

use super::group::GroupModel;

#[derive(Clone)]
pub struct UserModel {
    pub id: i32,
    pub username: String,
    pub hashed_password: String,
    pub nickname: String,
    pub avatar_uri: String,
    pub is_active: bool,
    pub groups: Option<Vec<GroupModel>>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// Here we've implemented `Debug` manually to avoid accidentally logging the
// password hash.
impl std::fmt::Debug for UserModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UserModel")
            .field("id", &self.id)
            .field("username", &self.username)
            .field("hashed_password", &"[redacted]")
            .field("nickname", &self.nickname)
            .field("avatar_uri", &self.avatar_uri)
            .field("is_active", &self.is_active)
            .field("groups", &self.groups)
            .field("created_at", &self.created_at)
            .field("updated_at", &self.updated_at)
            .finish()
    }
}
