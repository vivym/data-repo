use diesel::prelude::*;

use crate::domain::models::ds_item::DatasetItemModel;
use crate::infra::db::schema::ds_items;
use crate::infra::repositories::error::{RepoError, RepoResult, map_interact_error};
use super::schema::DatasetItemDB;

#[derive(AsChangeset)]
#[diesel(table_name = ds_items)]
pub struct UpdatedDatasetItemDB {
    pub typ: Option<String>,
    pub uri: Option<String>,
}

pub async fn update_by_id(
    db: &deadpool_diesel::postgres::Pool,
    item_id: i32,
    updated_item: UpdatedDatasetItemDB,
) -> RepoResult<DatasetItemModel> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    let res = conn
        .interact(move |conn| {
            diesel::update(
                ds_items::table
                    .filter(ds_items::id.eq(item_id))
            )
            .set(updated_item)
            .returning(DatasetItemDB::as_returning())
            .get_result(conn)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(RepoError::Diesel)?;

    Ok(res.into())
}
