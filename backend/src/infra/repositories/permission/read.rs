use diesel::prelude::*;
use serde::Deserialize;

use crate::domain::models::permission::PermissionModel;
use crate::infra::db::schema::permissions;
use crate::infra::repositories::{
    error::{RepoError, RepoResult, map_interact_error},
    default_skip,
    default_limit,
};
use super::schema::PermissionDB;

#[derive(Debug, Deserialize)]
pub struct PermissionsFilter {
    #[serde(default = "default_skip")]
    skip: i64,
    #[serde(default = "default_limit")]
    limit: i64,
}

pub async fn get_by_id(
    db: &deadpool_diesel::postgres::Pool,
    group_id: i32,
) -> RepoResult<PermissionModel> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    let res = conn
        .interact(move |conn| {
            permissions::table
                .filter(permissions::id.eq(group_id))
                .select(PermissionDB::as_select())
                .first(conn)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(RepoError::Diesel)?;

    Ok(res.into())
}

pub async fn try_get_by_id(
    db: &deadpool_diesel::postgres::Pool,
    group_id: i32,
) -> RepoResult<Option<PermissionModel>> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    let res = conn
        .interact(move |conn| {
            permissions::table
                .filter(permissions::id.eq(group_id))
                .select(PermissionDB::as_select())
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
) -> RepoResult<Option<PermissionModel>> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    let res = conn
        .interact(move |conn| {
            permissions::table
                .filter(permissions::name.eq(name))
                .select(PermissionDB::as_select())
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
    filter: PermissionsFilter,
) -> RepoResult<Vec<PermissionModel>> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    let res = conn
        .interact(move |conn| {
            permissions::table
                .into_boxed::<diesel::pg::Pg>()
                .offset(filter.skip)
                .limit(filter.limit)
                .select(PermissionDB::as_select())
                .load::<PermissionDB>(conn)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(RepoError::Diesel)?;

    let permissions: Vec<PermissionModel> = res
        .into_iter()
        .map(Into::into)
        .collect();

    Ok(permissions)
}
