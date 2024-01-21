use diesel::prelude::*;
use serde::Deserialize;

use crate::domain::models::ds_shard::DatasetShardModel;
use crate::infra::db::schema::{ds_shards, datasets_shards_rel, datasets};
use crate::infra::repositories::{
    error::{RepoError, RepoResult, map_interact_error},
    default_skip,
    default_limit,
};
use super::schema::DatasetShardDB;

#[derive(Debug, Deserialize)]
pub struct DatasetShardsFilter {
    ds_id: Option<i32>,
    #[serde(default = "default_skip")]
    skip: i64,
    #[serde(default = "default_limit")]
    limit: i64,
}

pub async fn get_by_id(
    db: &deadpool_diesel::postgres::Pool,
    shard_id: i32,
) -> RepoResult<DatasetShardModel> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    let res = conn
        .interact(move |conn| {
            ds_shards::table
                .filter(ds_shards::id.eq(shard_id))
                .select(DatasetShardDB::as_select())
                .first(conn)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(RepoError::Diesel)?;

    Ok(res.into())
}

pub async fn try_get_by_id(
    db: &deadpool_diesel::postgres::Pool,
    shard_id: i32,
) -> RepoResult<Option<DatasetShardModel>> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    let res = conn
        .interact(move |conn| {
            ds_shards::table
                .filter(ds_shards::id.eq(shard_id))
                .select(DatasetShardDB::as_select())
                .first(conn)
        })
        .await
        .map_err(map_interact_error)?;

    match res {
        Ok(res) => Ok(Some(res.into())),
        Err(diesel::NotFound) => Ok(None),
        Err(e) => Err(RepoError::Diesel(e)),
    }
}

pub async fn try_get_by_uri(
    db: &deadpool_diesel::postgres::Pool,
    uri: String,
) -> RepoResult<Option<DatasetShardModel>> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    let res = conn
        .interact(move |conn| {
            ds_shards::table
                .filter(ds_shards::uri.eq(uri))
                .select(DatasetShardDB::as_select())
                .first(conn)
        })
        .await
        .map_err(map_interact_error)?;

    match res {
        Ok(res) => Ok(Some(res.into())),
        Err(diesel::NotFound) => Ok(None),
        Err(e) => Err(RepoError::Diesel(e)),
    }
}

pub async fn get_all(
    db: &deadpool_diesel::postgres::Pool,
    filter: DatasetShardsFilter,
) -> RepoResult<Vec<DatasetShardModel>> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    let res = conn
        .interact(move |conn| {
            if let Some(ds_id) = filter.ds_id {
                ds_shards::table
                    .inner_join(datasets_shards_rel::table)
                    .inner_join(datasets::table.on(
                        datasets::id.eq(datasets_shards_rel::ds_id)
                    ))
                    .filter(datasets::id.eq(ds_id))
                    .offset(filter.skip)
                    .limit(filter.limit)
                    .select(DatasetShardDB::as_select())
                    .load::<DatasetShardDB>(conn)
            } else {
                ds_shards::table
                    .offset(filter.skip)
                    .limit(filter.limit)
                    .select(DatasetShardDB::as_select())
                    .load::<DatasetShardDB>(conn)
            }
        })
        .await
        .map_err(map_interact_error)?
        .map_err(RepoError::Diesel)?;

    let shards: Vec<DatasetShardModel> = res
        .into_iter()
        .map(Into::into)
        .collect();

    Ok(shards)
}
