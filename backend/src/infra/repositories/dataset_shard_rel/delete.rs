use diesel::prelude::*;

use crate::infra::db::schema::datasets_shards_rel;
use crate::infra::repositories::error::{RepoError, RepoResult, map_interact_error};

pub async fn delete_by_id(
    db: &deadpool_diesel::postgres::Pool,
    ds_id: i32,
    shard_id: i32,
) -> RepoResult<()> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    conn
        .interact(move |conn| {
            diesel::delete(
                datasets_shards_rel::table
                    .filter(datasets_shards_rel::ds_id.eq(ds_id))
                    .filter(datasets_shards_rel::shard_id.eq(shard_id))
            )
            .execute(conn)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(RepoError::Diesel)?;

    Ok(())
}
