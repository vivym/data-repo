use chrono::NaiveDateTime;

#[derive(Clone, Debug)]
pub struct DatasetModel {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
