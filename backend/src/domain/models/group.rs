use chrono::NaiveDateTime;

#[derive(Clone, Debug)]
pub struct GroupModel {
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
