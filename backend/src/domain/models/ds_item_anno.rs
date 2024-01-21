use chrono::NaiveDateTime;

#[derive(Clone, Debug)]
pub struct DatasetItemAnnoModel {
    pub id: i32,
    pub item_id: i32,
    pub name: String,
    pub typ: String,
    pub uri: Option<String>,
    pub number: Option<f64>,
    pub text: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
