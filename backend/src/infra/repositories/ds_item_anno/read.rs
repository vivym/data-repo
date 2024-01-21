use diesel::prelude::*;
use serde::Deserialize;

use crate::domain::models::ds_item_anno::DatasetItemAnnoModel;
use crate::infra::db::schema::{ds_item_annos, datasets_items_rel, datasets};
use crate::infra::repositories::{
    error::{RepoError, RepoResult, map_interact_error},
    default_skip,
    default_limit,
};
use super::schema::DatasetItemAnnoDB;

#[derive(Debug, Deserialize)]
pub struct DatasetItemAnnosFilter {
    ds_id: Option<i32>,
    item_id: Option<i32>,
    typ: Option<String>,
    #[serde(default = "default_skip")]
    skip: i64,
    #[serde(default = "default_limit")]
    limit: i64,
}

pub async fn get_by_id(
    db: &deadpool_diesel::postgres::Pool,
    anno_id: i32,
) -> RepoResult<DatasetItemAnnoModel> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    let res = conn
        .interact(move |conn| {
            ds_item_annos::table
                .filter(ds_item_annos::id.eq(anno_id))
                .select(DatasetItemAnnoDB::as_select())
                .first(conn)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(RepoError::Diesel)?;

    Ok(res.into())
}

pub async fn try_get_by_id(
    db: &deadpool_diesel::postgres::Pool,
    anno_id: i32,
) -> RepoResult<Option<DatasetItemAnnoModel>> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    let res = conn
        .interact(move |conn| {
            ds_item_annos::table
                .filter(ds_item_annos::id.eq(anno_id))
                .select(DatasetItemAnnoDB::as_select())
                .first(conn)
        })
        .await
        .map_err(map_interact_error)?;

    match res {
        Ok(res) => Ok(Some(res.into())),
        Err(diesel::NotFound) => Ok(None),
        Err(e) => Err(RepoError::Diesel(e)),
    }
}

pub async fn get_all(
    db: &deadpool_diesel::postgres::Pool,
    filter: DatasetItemAnnosFilter,
) -> RepoResult<Vec<DatasetItemAnnoModel>> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    let res = conn
        .interact(move |conn| {
            let mut query = ds_item_annos::table
                .into_boxed::<diesel::pg::Pg>();

            if let Some(item_id) = filter.item_id {
                query = query.filter(ds_item_annos::item_id.eq(item_id));
            }

            if let Some(typ) = filter.typ {
                query = query.filter(ds_item_annos::typ.eq(typ));
            }

            if let Some(ds_id) = filter.ds_id {
                query
                    .inner_join(datasets_items_rel::table.on(
                        datasets_items_rel::item_id.eq(ds_item_annos::item_id)
                    ))
                    .inner_join(datasets::table.on(
                        datasets::id.eq(datasets_items_rel::ds_id)
                    ))
                    .filter(datasets::id.eq(ds_id))
                    .offset(filter.skip)
                    .limit(filter.limit)
                    .select(DatasetItemAnnoDB::as_select())
                    .load::<DatasetItemAnnoDB>(conn)
            } else {
                query
                    .offset(filter.skip)
                    .limit(filter.limit)
                    .select(DatasetItemAnnoDB::as_select())
                    .load::<DatasetItemAnnoDB>(conn)
            }
        })
        .await
        .map_err(map_interact_error)?
        .map_err(RepoError::Diesel)?;

    let annos: Vec<DatasetItemAnnoModel> = res
        .into_iter()
        .map(Into::into)
        .collect();

    Ok(annos)
}
