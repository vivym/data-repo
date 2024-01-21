use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::domain::models::dataset_shard::DatasetShardModel;
use crate::infra::db::schema::datasets_shards_rel;

#[derive(Queryable, Selectable, Identifiable, Associations)]
#[diesel(primary_key(ds_id, shard_id))]
#[diesel(belongs_to(super::super::dataset::DatasetDB, foreign_key = ds_id))]
#[diesel(belongs_to(super::super::ds_shard::DatasetShardDB, foreign_key = shard_id))]
#[diesel(table_name = datasets_shards_rel)]     // Use the 'datasets_shards_rel' table
#[diesel(check_for_backend(diesel::pg::Pg))]    // Check compatibility with PostgreSQL
pub struct DatasetShardDB {
    pub ds_id: i32,
    pub shard_id: i32,
    pub created_at: NaiveDateTime,
}

impl Into<DatasetShardModel> for DatasetShardDB {
    fn into(self) -> DatasetShardModel {
        DatasetShardModel {
            ds_id: self.ds_id,
            shard_id: self.shard_id,
            created_at: self.created_at,
        }
    }
}
