use chrono::NaiveDateTime;

#[derive(Clone, Debug)]
pub struct GroupPermModel {
    pub group_id: i32,
    pub permission_id: i32,
    pub created_at: NaiveDateTime,
}
