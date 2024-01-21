use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::domain::models::dataset_item::DatasetItemModel;
use crate::infra::db::schema::datasets_items_rel;

#[derive(Queryable, Selectable, Identifiable, Associations)]
#[diesel(primary_key(ds_id, item_id))]
#[diesel(belongs_to(super::super::dataset::DatasetDB, foreign_key = ds_id))]
#[diesel(belongs_to(super::super::ds_item::DatasetItemDB, foreign_key = item_id))]
#[diesel(table_name = datasets_items_rel)]      // Use the 'datasets_items_rel' table
#[diesel(check_for_backend(diesel::pg::Pg))]    // Check compatibility with PostgreSQL
pub struct DatasetItemDB {
    pub ds_id: i32,
    pub item_id: i32,
    pub created_at: NaiveDateTime,
}

impl Into<DatasetItemModel> for DatasetItemDB {
    fn into(self) -> DatasetItemModel {
        DatasetItemModel {
            ds_id: self.ds_id,
            item_id: self.item_id,
            created_at: self.created_at,
        }
    }
}
