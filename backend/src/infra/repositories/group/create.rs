use diesel::prelude::*;
use serde::Deserialize;

use crate::domain::models::group::GroupModel;
use crate::infra::db::schema::groups;
use crate::infra::repositories::error::{RepoError, RepoResult, map_interact_error};
use super::schema::GroupDB;

#[derive(Deserialize, Insertable)]
#[diesel(table_name = groups)]
pub struct NewGroupDB {
    pub name: String,
}

pub async fn create(
    db: &deadpool_diesel::postgres::Pool,
    new_group: NewGroupDB,
) -> RepoResult<GroupModel> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    let res = conn
        .interact(|conn| {
            diesel::insert_into(groups::table)
                .values(new_group)
                .returning(GroupDB::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(RepoError::Diesel)?;

    Ok(res.into())
}
