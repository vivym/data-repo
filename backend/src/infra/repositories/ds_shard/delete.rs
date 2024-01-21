use diesel::prelude::*;

use crate::infra::db::schema::ds_shards;
use crate::infra::repositories::error::{RepoError, RepoResult, map_interact_error};

pub async fn delete_by_id(
    db: &deadpool_diesel::postgres::Pool,
    shard_id: i32,
) -> RepoResult<()> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    conn
        .interact(move |conn| {
            diesel::delete(
                ds_shards::table
                    .filter(ds_shards::id.eq(shard_id))
            )
            .execute(conn)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(RepoError::Diesel)?;

    Ok(())
}
