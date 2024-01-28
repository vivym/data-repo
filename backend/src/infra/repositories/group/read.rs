use std::collections::HashMap;

use diesel::prelude::*;
use serde::Deserialize;

use crate::domain::models::group::GroupModel;
use crate::infra::db::schema::{groups, groups_permissions_rel, permissions};
use crate::infra::repositories::permission::PermissionDB;
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

pub async fn get_by_ids(
    db: &deadpool_diesel::postgres::Pool,
    group_ids: Vec<i32>,
    with_permissions: bool,
) -> RepoResult<Vec<GroupModel>> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    let res = conn
        .interact(move |conn| {
            let query = groups::table
                .filter(groups::id.eq_any(&group_ids));

            if with_permissions {
                let groups_perms = query
                    .left_join(
                        groups_permissions_rel::table
                            .on(groups_permissions_rel::group_id.eq(groups::id))
                    )
                    .left_join(
                        permissions::table
                            .on(permissions::id.eq(groups_permissions_rel::permission_id))
                    )
                    .select((GroupDB::as_select(), Option::<PermissionDB>::as_select()))
                    .load::<(GroupDB, Option<PermissionDB>)>(conn)?;

                let mut groups = HashMap::<i32, GroupModel>::new();
                for (group, permission) in groups_perms {
                    let group = groups
                        .entry(group.id)
                        .or_insert_with(|| group.into());

                    if let Some(permission) = permission {
                        if let Some(permissions) = &mut group.permissions {
                            permissions.push(permission.into());
                        } else {
                            group.permissions = Some(vec![permission.into()]);
                        }
                    }
                }

                Ok(group_ids
                    .into_iter()
                    .map(|id| groups.remove(&id))
                    .collect::<Vec<Option<GroupModel>>>())
            } else {
                let groups = query
                    .select(GroupDB::as_select())
                    .load::<GroupDB>(conn)?;

                let mut groups_map = HashMap::<i32, GroupModel>::new();
                for group in groups {
                    groups_map.insert(group.id, group.into());
                }

                Ok(groups.into_iter().map(Into::into).map(|v| Some(v)).collect())
            }
        })
        .await
        .map_err(map_interact_error)?
        .map_err(RepoError::Diesel)?;

    Ok(res)
}
