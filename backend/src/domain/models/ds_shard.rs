use chrono::NaiveDateTime;

#[derive(Clone, Debug)]
pub struct DatasetShardModel {
    pub id: i32,
    pub uri: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
