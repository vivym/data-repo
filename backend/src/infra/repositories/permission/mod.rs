pub mod create;
pub mod delete;
pub mod read;
pub mod schema;
pub mod update;

pub use schema::PermissionDB;

pub use create::{
    NewPermissionDB,
    create,
};

pub use read::{
    PermissionsFilter,
    get_by_id,
    try_get_by_id,
    try_get_by_name,
    get_all,
};

pub use update::{
    UpdatedPermissionDB,
    update_by_id,
};

pub use delete::{delete_by_id, delete_by_ids};
