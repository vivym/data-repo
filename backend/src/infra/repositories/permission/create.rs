use diesel::prelude::*;
use serde::Deserialize;

use crate::domain::models::permission::PermissionModel;
use crate::infra::db::schema::permissions;
use crate::infra::repositories::error::{RepoError, RepoResult, map_interact_error};
use super::schema::PermissionDB;

#[derive(Deserialize, Insertable)]
#[diesel(table_name = permissions)]
pub struct NewPermissionDB {
    pub name: String,
}

pub async fn create(
    db: &deadpool_diesel::postgres::Pool,
    new_group: NewPermissionDB,
) -> RepoResult<PermissionModel> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    let res = conn
        .interact(|conn| {
            diesel::insert_into(permissions::table)
                .values(new_group)
                .returning(PermissionDB::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(RepoError::Diesel)?;

    Ok(res.into())
}
