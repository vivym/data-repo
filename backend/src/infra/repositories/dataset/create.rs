use diesel::prelude::*;
use serde::Deserialize;

use crate::domain::models::dataset::DatasetModel;
use crate::infra::db::schema::datasets;
use crate::infra::repositories::error::{RepoError, RepoResult, map_interact_error};
use super::schema::DatasetDB;

#[derive(Deserialize, Insertable)]
#[diesel(table_name = datasets)]
pub struct NewDatasetDB {
    pub name: String,
    pub description: String,
}

pub async fn create(
    db: &deadpool_diesel::postgres::Pool,
    new_ds: NewDatasetDB,
) -> RepoResult<DatasetModel> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    let res = conn
        .interact(|conn| {
            diesel::insert_into(datasets::table)
                .values(new_ds)
                .returning(DatasetDB::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(RepoError::Diesel)?;

    Ok(res.into())
}
