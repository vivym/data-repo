use chrono::NaiveDateTime;

#[derive(Clone, Debug)]
pub struct DatasetItemModel {
    pub ds_id: i32,
    pub item_id: i32,
    pub created_at: NaiveDateTime,
}
