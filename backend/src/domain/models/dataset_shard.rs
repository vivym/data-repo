use chrono::NaiveDateTime;

#[derive(Clone, Debug)]
pub struct DatasetShardModel {
    pub ds_id: i32,
    pub shard_id: i32,
    pub created_at: NaiveDateTime,
}
