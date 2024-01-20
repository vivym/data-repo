use diesel::prelude::*;

use crate::domain::models::permission::PermissionModel;
use crate::infra::db::schema::permissions;
use crate::infra::repositories::error::{RepoError, RepoResult, map_interact_error};
use super::schema::PermissionDB;

#[derive(AsChangeset)]
#[diesel(table_name = permissions)]
pub struct UpdatedPermissionDB {
    pub name: Option<String>,
}

pub async fn update_by_id(
    db: &deadpool_diesel::postgres::Pool,
    group_id: i32,
    updated_user: UpdatedPermissionDB,
) -> RepoResult<PermissionModel> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    let res = conn
        .interact(move |conn| {
            diesel::update(
                permissions::table
                    .filter(permissions::id.eq(group_id))
            )
            .set(updated_user)
            .returning(PermissionDB::as_returning())
            .get_result(conn)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(RepoError::Diesel)?;

    Ok(res.into())
}
