use diesel::prelude::*;
use serde::Deserialize;

use crate::domain::models::group::GroupModel;
use crate::infra::db::schema::groups;
use crate::infra::repositories::{
    error::{RepoError, RepoResult, map_interact_error},
    default_skip,
    default_limit,
};
use super::schema::GroupDB;

#[derive(Debug, Deserialize)]
pub struct GroupsFilter {
    #[serde(default = "default_skip")]
    skip: i64,
    #[serde(default = "default_limit")]
    limit: i64,
}

pub async fn get_by_id(
    db: &deadpool_diesel::postgres::Pool,
    group_id: i32,
) -> RepoResult<GroupModel> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    let res = conn
        .interact(move |conn| {
            groups::table
                .filter(groups::id.eq(group_id))
                .select(GroupDB::as_select())
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
) -> RepoResult<Option<GroupModel>> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    let res = conn
        .interact(move |conn| {
            groups::table
                .filter(groups::id.eq(group_id))
                .select(GroupDB::as_select())
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
    group_name: String,
) -> RepoResult<Option<GroupModel>> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    let res = conn
        .interact(move |conn| {
            groups::table
                .filter(groups::name.eq(group_name))
                .select(GroupDB::as_select())
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
    filter: GroupsFilter,
) -> RepoResult<Vec<GroupModel>> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    let res = conn
        .interact(move |conn| {
            groups::table
                .into_boxed::<diesel::pg::Pg>()
                .offset(filter.skip)
                .limit(filter.limit)
                .select(GroupDB::as_select())
                .load::<GroupDB>(conn)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(RepoError::Diesel)?;

    let groups: Vec<GroupModel> = res
        .into_iter()
        .map(Into::into)
        .collect();

    Ok(groups)
}
