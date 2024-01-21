use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::domain::models::dataset::DatasetModel;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DatasetSchema {
    pub id: i32,
    pub name: String,
    pub description: String,
    #[schema(value_type = String)]
    created_at: NaiveDateTime,
    #[schema(value_type = String)]
    updated_at: NaiveDateTime,
}

impl From<DatasetModel> for DatasetSchema {
    fn from(dataset: DatasetModel) -> Self {
        Self {
            id: dataset.id,
            name: dataset.name,
            description: dataset.description,
            created_at: dataset.created_at,
            updated_at: dataset.updated_at,
        }
    }
}
