pub mod create;
pub mod delete;
pub mod read;
pub mod schema;
pub mod update;

pub use schema::GroupDB;

pub use create::{
    NewGroupDB,
    create,
};

pub use read::{
    GroupsFilter,
    get_by_id,
    try_get_by_id,
    try_get_by_name,
    get_all,
    get_by_ids,
};

pub use update::{
    UpdatedGroupDB,
    update_by_id,
};

pub use delete::{delete_by_id, delete_by_ids};
