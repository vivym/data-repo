use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::domain::models::dataset::DatasetModel;
use crate::infra::db::schema::datasets;

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = datasets)]                // Use the 'datasets' table
#[diesel(check_for_backend(diesel::pg::Pg))]    // Check compatibility with PostgreSQL
pub struct DatasetDB {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Into<DatasetModel> for DatasetDB {
    fn into(self) -> DatasetModel {
        DatasetModel {
            id: self.id,
            name: self.name,
            description: self.description,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
