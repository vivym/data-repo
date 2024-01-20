use chrono::NaiveDateTime;

#[derive(Clone, Debug)]
pub struct UserGroupModel {
    pub user_id: i32,
    pub group_id: i32,
    pub created_at: NaiveDateTime,
}
