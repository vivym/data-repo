use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::domain::models::ds_shard::DatasetShardModel;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DatasetShardSchema {
    pub id: i32,
    pub uri: String,
    #[schema(value_type = String)]
    created_at: NaiveDateTime,
    #[schema(value_type = String)]
    updated_at: NaiveDateTime,
}

impl From<DatasetShardModel> for DatasetShardSchema {
    fn from(shard: DatasetShardModel) -> Self {
        Self {
            id: shard.id,
            uri: shard.uri,
            created_at: shard.created_at,
            updated_at: shard.updated_at,
        }
    }
}
