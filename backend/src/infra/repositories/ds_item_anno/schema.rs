use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::domain::models::ds_item_anno::DatasetItemAnnoModel;
use crate::infra::db::schema::ds_item_annos;

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = ds_item_annos)]           // Use the 'ds_item_annos' table
#[diesel(check_for_backend(diesel::pg::Pg))]    // Check compatibility with PostgreSQL
pub struct DatasetItemAnnoDB {
    pub id: i32,
    pub item_id: i32,
    pub name: String,
    pub typ: String,
    pub uri: Option<String>,
    pub number: Option<f64>,
    pub text: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Into<DatasetItemAnnoModel> for DatasetItemAnnoDB {
    fn into(self) -> DatasetItemAnnoModel {
        DatasetItemAnnoModel {
            id: self.id,
            item_id: self.item_id,
            name: self.name,
            typ: self.typ,
            uri: self.uri,
            number: self.number,
            text: self.text,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
