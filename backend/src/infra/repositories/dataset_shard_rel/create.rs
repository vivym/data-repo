use diesel::prelude::*;
use serde::Deserialize;

use crate::domain::models::dataset_shard::DatasetShardModel;
use crate::infra::db::schema::datasets_shards_rel;
use crate::infra::repositories::error::{RepoError, RepoResult, map_interact_error};
use super::schema::DatasetShardDB;

#[derive(Deserialize, Insertable)]
#[diesel(table_name = datasets_shards_rel)]
pub struct NewDatasetShardDB {
    pub ds_id: i32,
    pub shard_id: i32,
}

pub async fn create(
    db: &deadpool_diesel::postgres::Pool,
    new_ds_shard: NewDatasetShardDB,
) -> RepoResult<DatasetShardModel> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    let res = conn
        .interact(|conn| {
            diesel::insert_into(datasets_shards_rel::table)
                .values(new_ds_shard)
                .returning(DatasetShardDB::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(RepoError::Diesel)?;

    Ok(res.into())
}
