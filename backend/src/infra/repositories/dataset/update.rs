use diesel::prelude::*;

use crate::domain::models::dataset::DatasetModel;
use crate::infra::db::schema::datasets;
use crate::infra::repositories::error::{RepoError, RepoResult, map_interact_error};
use super::schema::DatasetDB;

#[derive(AsChangeset)]
#[diesel(table_name = datasets)]
pub struct UpdatedDatasetDB {
    pub description: Option<String>,
}

pub async fn update_by_id(
    db: &deadpool_diesel::postgres::Pool,
    ds_id: i32,
    updated_ds: UpdatedDatasetDB,
) -> RepoResult<DatasetModel> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    let res = conn
        .interact(move |conn| {
            diesel::update(
                datasets::table
                    .filter(datasets::id.eq(ds_id))
            )
            .set(updated_ds)
            .returning(DatasetDB::as_returning())
            .get_result(conn)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(RepoError::Diesel)?;

    Ok(res.into())
}
