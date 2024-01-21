use diesel::prelude::*;

use crate::infra::db::schema::ds_item_annos;
use crate::infra::repositories::error::{RepoError, RepoResult, map_interact_error};

pub async fn delete_by_id(
    db: &deadpool_diesel::postgres::Pool,
    anno_id: i32,
) -> RepoResult<()> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    conn
        .interact(move |conn| {
            diesel::delete(
                ds_item_annos::table
                    .filter(ds_item_annos::id.eq(anno_id))
            )
            .execute(conn)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(RepoError::Diesel)?;

    Ok(())
}
