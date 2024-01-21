use diesel::prelude::*;
use serde::Deserialize;

use crate::domain::models::ds_item::DatasetItemModel;
use crate::infra::db::schema::{ds_items, datasets, datasets_items_rel};
use crate::infra::repositories::{
    error::{RepoError, RepoResult, map_interact_error},
    default_skip,
    default_limit,
};
use super::schema::DatasetItemDB;

#[derive(Debug, Deserialize)]
pub struct DatasetItemsFilter {
    ds_id: Option<i32>,
    #[serde(default = "default_skip")]
    skip: i64,
    #[serde(default = "default_limit")]
    limit: i64,
}

pub async fn get_by_id(
    db: &deadpool_diesel::postgres::Pool,
    item_id: i32,
) -> RepoResult<DatasetItemModel> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    let res = conn
        .interact(move |conn| {
            ds_items::table
                .filter(ds_items::id.eq(item_id))
                .select(DatasetItemDB::as_select())
                .first(conn)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(RepoError::Diesel)?;

    Ok(res.into())
}

pub async fn try_get_by_id(
    db: &deadpool_diesel::postgres::Pool,
    item_id: i32,
) -> RepoResult<Option<DatasetItemModel>> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    let res = conn
        .interact(move |conn| {
            ds_items::table
                .filter(ds_items::id.eq(item_id))
                .select(DatasetItemDB::as_select())
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
) -> RepoResult<Option<DatasetItemModel>> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    let res = conn
        .interact(move |conn| {
            ds_items::table
                .filter(ds_items::uri.eq(uri))
                .select(DatasetItemDB::as_select())
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
    filter: DatasetItemsFilter,
) -> RepoResult<Vec<DatasetItemModel>> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    let res = conn
        .interact(move |conn| {
            if let Some(ds_id) = filter.ds_id {
                ds_items::table
                    .inner_join(datasets_items_rel::table)
                    .inner_join(datasets::table.on(
                        datasets::id.eq(datasets_items_rel::ds_id)
                    ))
                    .filter(datasets::id.eq(ds_id))
                    .offset(filter.skip)
                    .limit(filter.limit)
                    .select(DatasetItemDB::as_select())
                    .load::<DatasetItemDB>(conn)
            } else {
                ds_items::table
                    .offset(filter.skip)
                    .limit(filter.limit)
                    .select(DatasetItemDB::as_select())
                    .load::<DatasetItemDB>(conn)
            }
        })
        .await
        .map_err(map_interact_error)?
        .map_err(RepoError::Diesel)?;

    let items: Vec<DatasetItemModel> = res
        .into_iter()
        .map(Into::into)
        .collect();

    Ok(items)
}
