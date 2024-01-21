use diesel::prelude::*;
use serde::Deserialize;

use crate::domain::models::ds_item::DatasetItemModel;
use crate::infra::db::schema::ds_items;
use crate::infra::repositories::error::{RepoError, RepoResult, map_interact_error};
use super::schema::DatasetItemDB;

#[derive(Deserialize, Insertable)]
#[diesel(table_name = ds_items)]
pub struct NewDatasetItemDB {
    pub typ: String,
    pub uri: String,
}

pub async fn create(
    db: &deadpool_diesel::postgres::Pool,
    new_item: NewDatasetItemDB,
) -> RepoResult<DatasetItemModel> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    let res = conn
        .interact(|conn| {
            diesel::insert_into(ds_items::table)
                .values(new_item)
                .returning(DatasetItemDB::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(RepoError::Diesel)?;

    Ok(res.into())
}
