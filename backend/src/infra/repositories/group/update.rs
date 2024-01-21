use diesel::prelude::*;

use crate::domain::models::group::GroupModel;
use crate::infra::db::schema::groups;
use crate::infra::repositories::error::{RepoError, RepoResult, map_interact_error};
use super::schema::GroupDB;

#[derive(AsChangeset)]
#[diesel(table_name = groups)]
pub struct UpdatedGroupDB {
    pub name: Option<String>,
}

pub async fn update_by_id(
    db: &deadpool_diesel::postgres::Pool,
    group_id: i32,
    updated_group: UpdatedGroupDB,
) -> RepoResult<GroupModel> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    let res = conn
        .interact(move |conn| {
            diesel::update(
                groups::table
                    .filter(groups::id.eq(group_id))
            )
            .set(updated_group)
            .returning(GroupDB::as_returning())
            .get_result(conn)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(RepoError::Diesel)?;

    Ok(res.into())
}
