use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::domain::models::ds_item::DatasetItemModel;
use crate::infra::db::schema::ds_items;

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = ds_items)]                // Use the 'ds_items' table
#[diesel(check_for_backend(diesel::pg::Pg))]    // Check compatibility with PostgreSQL
pub struct DatasetItemDB {
    pub id: i32,
    pub typ: String,
    pub uri: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Into<DatasetItemModel> for DatasetItemDB {
    fn into(self) -> DatasetItemModel {
        DatasetItemModel {
            id: self.id,
            typ: self.typ,
            uri: self.uri,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
