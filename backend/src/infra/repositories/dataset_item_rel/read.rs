use diesel::prelude::*;
use serde::Deserialize;

use crate::domain::models::dataset_item::DatasetItemModel;
use crate::infra::db::schema::datasets_items_rel;
use crate::infra::repositories::{
    error::{RepoError, RepoResult, map_interact_error},
    default_skip,
    default_limit,
};
use super::schema::DatasetItemDB;

#[derive(Debug, Deserialize)]
pub struct DatasetsItemsFilter {
    ds_id: Option<i32>,
    item_id: Option<i32>,
    #[serde(default = "default_skip")]
    skip: i64,
    #[serde(default = "default_limit")]
    limit: i64,
}

pub async fn get_by_id(
    db: &deadpool_diesel::postgres::Pool,
    ds_id: i32,
    item_id: i32,
) -> RepoResult<DatasetItemModel> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    let res = conn
        .interact(move |conn| {
            datasets_items_rel::table
                .filter(datasets_items_rel::ds_id.eq(ds_id))
                .filter(datasets_items_rel::item_id.eq(item_id))
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
    ds_id: i32,
    item_id: i32,
) -> RepoResult<Option<DatasetItemModel>> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    let res = conn
        .interact(move |conn| {
            datasets_items_rel::table
                .filter(datasets_items_rel::ds_id.eq(ds_id))
                .filter(datasets_items_rel::item_id.eq(item_id))
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
    filter: DatasetsItemsFilter,
) -> RepoResult<Vec<DatasetItemModel>> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    let res = conn
        .interact(move |conn| {
            let mut query = datasets_items_rel::table
                .into_boxed::<diesel::pg::Pg>();

            if let Some(ds_id) = filter.ds_id {
                query = query.filter(datasets_items_rel::ds_id.eq(ds_id));
            }

            if let Some(item_id) = filter.item_id {
                query = query.filter(datasets_items_rel::item_id.eq(item_id));
            }

            query
                .offset(filter.skip)
                .limit(filter.limit)
                .select(DatasetItemDB::as_select())
                .load::<DatasetItemDB>(conn)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(RepoError::Diesel)?;

    let datasets_items: Vec<DatasetItemModel> = res
        .into_iter()
        .map(Into::into)
        .collect();

    Ok(datasets_items)
}
