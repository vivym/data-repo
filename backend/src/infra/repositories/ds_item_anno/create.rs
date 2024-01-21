use diesel::prelude::*;
use serde::Deserialize;

use crate::domain::models::ds_item_anno::DatasetItemAnnoModel;
use crate::infra::db::schema::ds_item_annos;
use crate::infra::repositories::error::{RepoError, RepoResult, map_interact_error};
use super::schema::DatasetItemAnnoDB;

#[derive(Deserialize, Insertable)]
#[diesel(table_name = ds_item_annos)]
pub struct NewDatasetItemAnnoDB {
    pub name: String,
    pub typ: String,
    pub uri: Option<String>,
    pub number: Option<f64>,
    pub text: Option<String>,
}

pub async fn create(
    db: &deadpool_diesel::postgres::Pool,
    new_anno: NewDatasetItemAnnoDB,
) -> RepoResult<DatasetItemAnnoModel> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    let res = conn
        .interact(|conn| {
            diesel::insert_into(ds_item_annos::table)
                .values(new_anno)
                .returning(DatasetItemAnnoDB::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(RepoError::Diesel)?;

    Ok(res.into())
}
