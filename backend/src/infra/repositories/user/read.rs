use std::collections::HashSet;

use diesel::prelude::*;
use serde::Deserialize;

use crate::domain::models::user::UserModel;
use crate::infra::db::schema::{groups, groups_permissions_rel, permissions, users};
use crate::infra::repositories::{
    self,
    error::{RepoError, RepoResult, map_interact_error},
    default_skip,
    default_limit,
    user_group_rel::UserGroupDB,
    group::GroupDB,
    group_permission_rel::GroupPermDB,
    permission::PermissionDB,
};
use super::schema::UserDB;

#[derive(Debug, Deserialize)]
pub struct UsersFilter {
    username: Option<String>,
    nickname: Option<String>,
    is_active: Option<bool>,
    with_groups: Option<bool>,
    with_permissions: Option<bool>,
    #[serde(default = "default_skip")]
    skip: i64,
    #[serde(default = "default_limit")]
    limit: i64,
}

pub async fn get_by_id(
    db: &deadpool_diesel::postgres::Pool,
    user_id: i32,
) -> RepoResult<UserModel> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    let res = conn
        .interact(move |conn| {
            users::table
                .filter(users::id.eq(user_id))
                .select(UserDB::as_select())
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
) -> RepoResult<Option<UserModel>> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    let res = conn
        .interact(move |conn| {
            users::table
                .filter(users::id.eq(user_id))
                .select(UserDB::as_select())
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

pub async fn try_get_by_username(
    db: &deadpool_diesel::postgres::Pool,
    username: String,
) -> RepoResult<Option<UserModel>> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    let res = conn
        .interact(move |conn| {
            users::table
                .filter(users::username.eq(username))
                .select(UserDB::as_select())
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
    filter: UsersFilter,
) -> RepoResult<Vec<UserModel>> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    let users = conn
        .interact(move |conn| {
            let mut query = users::table
                .into_boxed::<diesel::pg::Pg>();

            if let Some(username) = filter.username {
                query = query.filter(users::username.eq(username));
            }

            if let Some(nickname) = filter.nickname {
                query = query.filter(users::nickname.eq(nickname));
            }

            if let Some(is_active) = filter.is_active {
                query = query.filter(users::is_active.eq(is_active));
            }

            query
                .offset(filter.skip)
                .limit(filter.limit)
                .select(UserDB::as_select())
                .load::<UserDB>(conn)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(RepoError::Diesel)?;

        let mut users: Vec<UserModel> = users
            .into_iter()
            .map(Into::into)
            .collect();

        let user_ids = users
            .iter()
            .map(|u| u.id)
            .collect::<Vec<i32>>();

        if let Some(true) = filter.with_groups {
            let group_ids = user_ids.clone(); // TODO: get groups from user ids

            let groups_per_user = repositories::group::get_by_ids(
                db, group_ids, filter.with_permissions.unwrap_or(false)
            )
                .await?;

            users
                .iter_mut()
                .zip(groups_per_user)
                .for_each(|(u, g)| {
                    u.groups = g;
                });
        }

    Ok(users)
}
