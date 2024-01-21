use diesel::prelude::*;

use crate::infra::db::schema::groups_permissions_rel;
use crate::infra::repositories::error::{RepoError, RepoResult, map_interact_error};

pub async fn delete_by_id(
    db: &deadpool_diesel::postgres::Pool,
    group_id: i32,
    perm_id: i32,
) -> RepoResult<()> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    conn
        .interact(move |conn| {
            diesel::delete(
                groups_permissions_rel::table
                    .filter(groups_permissions_rel::group_id.eq(group_id))
                    .filter(groups_permissions_rel::permission_id.eq(perm_id))
            )
            .execute(conn)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(RepoError::Diesel)?;

    Ok(())
}
