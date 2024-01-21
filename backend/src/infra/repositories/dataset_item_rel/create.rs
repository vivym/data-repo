use diesel::prelude::*;
use serde::Deserialize;

use crate::domain::models::dataset_item::DatasetItemModel;
use crate::infra::db::schema::datasets_items_rel;
use crate::infra::repositories::error::{RepoError, RepoResult, map_interact_error};
use super::schema::DatasetItemDB;

#[derive(Deserialize, Insertable)]
#[diesel(table_name = datasets_items_rel)]
pub struct NewDatasetItemDB {
    pub ds_id: i32,
    pub item_id: i32,
}

pub async fn create(
    db: &deadpool_diesel::postgres::Pool,
    new_ds_item: NewDatasetItemDB,
) -> RepoResult<DatasetItemModel> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    let res = conn
        .interact(|conn| {
            diesel::insert_into(datasets_items_rel::table)
                .values(new_ds_item)
                .returning(DatasetItemDB::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(RepoError::Diesel)?;

    Ok(res.into())
}
