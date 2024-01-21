use diesel::prelude::*;

use crate::infra::db::schema::datasets_items_rel;
use crate::infra::repositories::error::{RepoError, RepoResult, map_interact_error};

pub async fn delete_by_id(
    db: &deadpool_diesel::postgres::Pool,
    ds_id: i32,
    item_id: i32,
) -> RepoResult<()> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    conn
        .interact(move |conn| {
            diesel::delete(
                datasets_items_rel::table
                    .filter(datasets_items_rel::ds_id.eq(ds_id))
                    .filter(datasets_items_rel::item_id.eq(item_id))
            )
            .execute(conn)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(RepoError::Diesel)?;

    Ok(())
}
