use diesel::prelude::*;
use serde::Deserialize;

use crate::domain::models::user_group::UserGroupModel;
use crate::infra::db::schema::users_groups_rel;
use crate::infra::repositories::{
    error::{RepoError, RepoResult, map_interact_error},
    default_skip,
    default_limit,
};
use super::schema::UserGroupDB;

#[derive(Debug, Deserialize)]
pub struct UsersGroupsFilter {
    #[serde(default = "default_skip")]
    skip: i64,
    #[serde(default = "default_limit")]
    limit: i64,
}

pub async fn get_by_id(
    db: &deadpool_diesel::postgres::Pool,
    user_id: i32,
    group_id: i32,
) -> RepoResult<UserGroupModel> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    let res = conn
        .interact(move |conn| {
            users_groups_rel::table
                .filter(users_groups_rel::user_id.eq(user_id))
                .filter(users_groups_rel::group_id.eq(group_id))
                .select(UserGroupDB::as_select())
                .first(conn)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(RepoError::Diesel)?;

    Ok(res.into())
}

pub async fn try_get_by_id(
    db: &deadpool_diesel::postgres::Pool,
    user_id: i32,
    group_id: i32,
) -> RepoResult<Option<UserGroupModel>> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    let res = conn
        .interact(move |conn| {
            users_groups_rel::table
                .filter(users_groups_rel::user_id.eq(user_id))
                .filter(users_groups_rel::group_id.eq(group_id))
                .select(UserGroupDB::as_select())
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
    filter: UsersGroupsFilter,
) -> RepoResult<Vec<UserGroupModel>> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    let res = conn
        .interact(move |conn| {
            users_groups_rel::table
                .into_boxed::<diesel::pg::Pg>()
                .offset(filter.skip)
                .limit(filter.limit)
                .select(UserGroupDB::as_select())
                .load::<UserGroupDB>(conn)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(RepoError::Diesel)?;

    let users_groups: Vec<UserGroupModel> = res
        .into_iter()
        .map(Into::into)
        .collect();

    Ok(users_groups)
}
