use chrono::NaiveDateTime;

#[derive(Clone, Debug)]
pub struct DatasetItemModel {
    pub id: i32,
    pub typ: String,
    pub uri: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
