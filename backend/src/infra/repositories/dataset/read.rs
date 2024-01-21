use diesel::prelude::*;
use serde::Deserialize;

use crate::domain::models::dataset::DatasetModel;
use crate::infra::db::schema::datasets;
use crate::infra::repositories::{
    error::{RepoError, RepoResult, map_interact_error},
    default_skip,
    default_limit,
};
use super::schema::DatasetDB;

#[derive(Debug, Deserialize)]
pub struct DatasetsFilter {
    #[serde(default = "default_skip")]
    skip: i64,
    #[serde(default = "default_limit")]
    limit: i64,
}

pub async fn get_by_id(
    db: &deadpool_diesel::postgres::Pool,
    ds_id: i32,
) -> RepoResult<DatasetModel> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    let res = conn
        .interact(move |conn| {
            datasets::table
                .filter(datasets::id.eq(ds_id))
                .select(DatasetDB::as_select())
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
) -> RepoResult<Option<DatasetModel>> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    let res = conn
        .interact(move |conn| {
            datasets::table
                .filter(datasets::id.eq(ds_id))
                .select(DatasetDB::as_select())
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

pub async fn try_get_by_name(
    db: &deadpool_diesel::postgres::Pool,
    name: String,
) -> RepoResult<Option<DatasetModel>> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    let res = conn
        .interact(move |conn| {
            datasets::table
                .filter(datasets::name.eq(name))
                .select(DatasetDB::as_select())
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
    filter: DatasetsFilter,
) -> RepoResult<Vec<DatasetModel>> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    let res = conn
        .interact(move |conn| {
            datasets::table
                .into_boxed::<diesel::pg::Pg>()
                .offset(filter.skip)
                .limit(filter.limit)
                .select(DatasetDB::as_select())
                .load::<DatasetDB>(conn)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(RepoError::Diesel)?;

    let datasets: Vec<DatasetModel> = res
        .into_iter()
        .map(Into::into)
        .collect();

    Ok(datasets)
}
