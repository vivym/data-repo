use diesel::prelude::*;
use serde::Deserialize;

use crate::domain::models::group_perm::GroupPermModel;
use crate::infra::db::schema::groups_permissions;
use crate::infra::repositories::error::{RepoError, RepoResult, map_interact_error};
use super::schema::GroupPermDB;

#[derive(Deserialize, Insertable)]
#[diesel(table_name = groups_permissions)]
pub struct NewGroupPermDB {
    pub group_id: i32,
    pub permission_id: i32,
}

pub async fn create(
    db: &deadpool_diesel::postgres::Pool,
    new_group_perm: NewGroupPermDB,
) -> RepoResult<GroupPermModel> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    let res = conn
        .interact(|conn| {
            diesel::insert_into(groups_permissions::table)
                .values(new_group_perm)
                .returning(GroupPermDB::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(RepoError::Diesel)?;

    Ok(res.into())
}
