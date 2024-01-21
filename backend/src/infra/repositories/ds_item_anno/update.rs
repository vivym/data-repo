use diesel::prelude::*;

use crate::domain::models::ds_item_anno::DatasetItemAnnoModel;
use crate::infra::db::schema::ds_item_annos;
use crate::infra::repositories::error::{RepoError, RepoResult, map_interact_error};
use super::schema::DatasetItemAnnoDB;

#[derive(AsChangeset)]
#[diesel(table_name = ds_item_annos)]
pub struct UpdatedDatasetItemAnnoDB {
    pub name: String,
    pub typ: String,
    pub uri: Option<String>,
    pub number: Option<f64>,
    pub text: Option<String>,
}

pub async fn update_by_id(
    db: &deadpool_diesel::postgres::Pool,
    anno_id: i32,
    updated_anno: UpdatedDatasetItemAnnoDB,
) -> RepoResult<DatasetItemAnnoModel> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    let res = conn
        .interact(move |conn| {
            diesel::update(
                ds_item_annos::table
                    .filter(ds_item_annos::id.eq(anno_id))
            )
            .set(updated_anno)
            .returning(DatasetItemAnnoDB::as_returning())
            .get_result(conn)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(RepoError::Diesel)?;

    Ok(res.into())
}
