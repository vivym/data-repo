use chrono::NaiveDateTime;

#[derive(Clone)]
pub struct UserModel {
    pub id: i32,
    pub username: String,
    pub hashed_password: String,
    pub nickname: String,
    pub avatar_uri: String,
    pub verified: bool,
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
            .field("verified", &self.verified)
            .field("created_at", &self.created_at)
            .field("updated_at", &self.updated_at)
            .finish()
    }
}
