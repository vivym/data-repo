use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::domain::models::ds_item::DatasetItemModel;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DatasetItemSchema {
    pub id: i32,
    pub typ: String,
    pub uri: String,
    #[schema(value_type = String)]
    created_at: NaiveDateTime,
    #[schema(value_type = String)]
    updated_at: NaiveDateTime,
}

impl From<DatasetItemModel> for DatasetItemSchema {
    fn from(item: DatasetItemModel) -> Self {
        Self {
            id: item.id,
            typ: item.typ,
            uri: item.uri,
            created_at: item.created_at,
            updated_at: item.updated_at,
        }
    }
}
