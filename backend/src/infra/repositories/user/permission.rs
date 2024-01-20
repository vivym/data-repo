use diesel::prelude::*;

use crate::domain::models::permission::PermissionModel;
use crate::infra::db::schema::{
    users,
    users_groups,
    groups_permissions,
    permissions,
};
use crate::infra::repositories::{
    error::{RepoError, RepoResult, map_interact_error},
    permission::PermissionDB,
};

pub async fn get_permissions(
    db: &deadpool_diesel::postgres::Pool,
    user_id: i32,
) -> RepoResult<Vec<PermissionModel>> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    let res = conn
        .interact(move |conn| {
            users::table
                .inner_join(users_groups::table)
                .inner_join(groups_permissions::table.on(
                    groups_permissions::group_id.eq(users_groups::group_id)
                ))
                .inner_join(permissions::table.on(
                    permissions::id.eq(groups_permissions::permission_id)
                ))
                .filter(users::id.eq(user_id))
                .select(PermissionDB::as_select())
                .distinct()
                .load::<PermissionDB>(conn)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(RepoError::Diesel)?;

    Ok(res.into_iter().map(Into::into).collect())
}
