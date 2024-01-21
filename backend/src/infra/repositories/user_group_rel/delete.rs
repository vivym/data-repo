use diesel::prelude::*;

use crate::infra::db::schema::users_groups_rel;
use crate::infra::repositories::error::{RepoError, RepoResult, map_interact_error};

pub async fn delete_by_id(
    db: &deadpool_diesel::postgres::Pool,
    user_id: i32,
    group_id: i32,
) -> RepoResult<()> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    conn
        .interact(move |conn| {
            diesel::delete(
                users_groups_rel::table
                    .filter(users_groups_rel::user_id.eq(user_id))
                    .filter(users_groups_rel::group_id.eq(group_id))
            )
            .execute(conn)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(RepoError::Diesel)?;

    Ok(())
}
