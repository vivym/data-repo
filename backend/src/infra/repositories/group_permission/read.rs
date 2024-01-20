use diesel::prelude::*;
use serde::Deserialize;

use crate::domain::models::group_perm::GroupPermModel;
use crate::infra::db::schema::groups_permissions;
use crate::infra::repositories::{
    error::{RepoError, RepoResult, map_interact_error},
    default_skip,
    default_limit,
};
use super::schema::GroupPermDB;

#[derive(Debug, Deserialize)]
pub struct GroupsPermsFilter {
    #[serde(default = "default_skip")]
    skip: i64,
    #[serde(default = "default_limit")]
    limit: i64,
}

pub async fn get_by_id(
    db: &deadpool_diesel::postgres::Pool,
    group_id: i32,
    perm_id: i32,
) -> RepoResult<GroupPermModel> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    let res = conn
        .interact(move |conn| {
            groups_permissions::table
                .filter(groups_permissions::group_id.eq(group_id))
                .filter(groups_permissions::permission_id.eq(perm_id))
                .select(GroupPermDB::as_select())
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
    perm_id: i32,
) -> RepoResult<Option<GroupPermModel>> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    let res = conn
        .interact(move |conn| {
            groups_permissions::table
                .filter(groups_permissions::group_id.eq(group_id))
                .filter(groups_permissions::permission_id.eq(perm_id))
                .select(GroupPermDB::as_select())
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
    filter: GroupsPermsFilter,
) -> RepoResult<Vec<GroupPermModel>> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    let res = conn
        .interact(move |conn| {
            groups_permissions::table
                .into_boxed::<diesel::pg::Pg>()
                .offset(filter.skip)
                .limit(filter.limit)
                .select(GroupPermDB::as_select())
                .load::<GroupPermDB>(conn)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(RepoError::Diesel)?;

    let groups_permissions: Vec<GroupPermModel> = res
        .into_iter()
        .map(Into::into)
        .collect();

    Ok(groups_permissions)
}
