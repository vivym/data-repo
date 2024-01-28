use chrono::NaiveDateTime;

use super::permission::PermissionModel;

#[derive(Clone, Debug)]
pub struct GroupModel {
    pub id: i32,
    pub name: String,
    pub permissions: Option<Vec<PermissionModel>>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
