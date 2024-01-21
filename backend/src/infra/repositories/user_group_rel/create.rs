use diesel::prelude::*;
use serde::Deserialize;

use crate::domain::models::user_group::UserGroupModel;
use crate::infra::db::schema::users_groups_rel;
use crate::infra::repositories::error::{RepoError, RepoResult, map_interact_error};
use super::schema::UserGroupDB;

#[derive(Deserialize, Insertable)]
#[diesel(table_name = users_groups_rel)]
pub struct NewUserGroupDB {
    pub user_id: i32,
    pub group_id: i32,
}

pub async fn create(
    db: &deadpool_diesel::postgres::Pool,
    new_user_group: NewUserGroupDB,
) -> RepoResult<UserGroupModel> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    let res = conn
        .interact(|conn| {
            diesel::insert_into(users_groups_rel::table)
                .values(new_user_group)
                .returning(UserGroupDB::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(RepoError::Diesel)?;

    Ok(res.into())
}
