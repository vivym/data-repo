use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::domain::models::ds_shard::DatasetShardModel;
use crate::infra::db::schema::ds_shards;

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = ds_shards)]               // Use the 'ds_shards' table
#[diesel(check_for_backend(diesel::pg::Pg))]    // Check compatibility with PostgreSQL
pub struct DatasetShardDB {
    pub id: i32,
    pub uri: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Into<DatasetShardModel> for DatasetShardDB {
    fn into(self) -> DatasetShardModel {
        DatasetShardModel {
            id: self.id,
            uri: self.uri,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
