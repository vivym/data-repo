use diesel::prelude::*;
use serde::Deserialize;

use crate::domain::models::ds_shard::DatasetShardModel;
use crate::infra::db::schema::ds_shards;
use crate::infra::repositories::error::{RepoError, RepoResult, map_interact_error};
use super::schema::DatasetShardDB;

#[derive(Deserialize, Insertable)]
#[diesel(table_name = ds_shards)]
pub struct NewDatasetShardDB {
    pub uri: String,
}

pub async fn create(
    db: &deadpool_diesel::postgres::Pool,
    new_shard: NewDatasetShardDB,
) -> RepoResult<DatasetShardModel> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    let res = conn
        .interact(|conn| {
            diesel::insert_into(ds_shards::table)
                .values(new_shard)
                .returning(DatasetShardDB::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(RepoError::Diesel)?;

    Ok(res.into())
}
