use diesel::prelude::*;

use crate::domain::models::ds_shard::DatasetShardModel;
use crate::infra::db::schema::ds_shards;
use crate::infra::repositories::error::{RepoError, RepoResult, map_interact_error};
use super::schema::DatasetShardDB;

#[derive(AsChangeset)]
#[diesel(table_name = ds_shards)]
pub struct UpdatedDatasetShardDB {
    pub uri: Option<String>,
}

pub async fn update_by_id(
    db: &deadpool_diesel::postgres::Pool,
    shard_id: i32,
    updated_shard: UpdatedDatasetShardDB,
) -> RepoResult<DatasetShardModel> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    let res = conn
        .interact(move |conn| {
            diesel::update(
                ds_shards::table
                    .filter(ds_shards::id.eq(shard_id))
            )
            .set(updated_shard)
            .returning(DatasetShardDB::as_returning())
            .get_result(conn)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(RepoError::Diesel)?;

    Ok(res.into())
}
