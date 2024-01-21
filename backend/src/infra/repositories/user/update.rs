use diesel::prelude::*;

use crate::domain::models::user::UserModel;
use crate::infra::db::schema::users;
use crate::infra::repositories::error::{RepoError, RepoResult, map_interact_error};
use super::schema::UserDB;

#[derive(AsChangeset)]
#[diesel(table_name = users)]
pub struct UpdatedUserDB {
    pub hashed_password: Option<String>,
    pub nickname: Option<String>,
    pub avatar_uri: Option<String>,
}

pub async fn update_by_id(
    db: &deadpool_diesel::postgres::Pool,
    user_id: i32,
    updated_user: UpdatedUserDB,
) -> RepoResult<UserModel> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    let res = conn
        .interact(move |conn| {
            diesel::update(
                users::table
                    .filter(users::id.eq(user_id))
            )
            .set(updated_user)
            .returning(UserDB::as_returning())
            .get_result(conn)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(RepoError::Diesel)?;

    Ok(res.into())
}

pub async fn activate_by_id(
    db: &deadpool_diesel::postgres::Pool,
    user_id: i32,
) -> RepoResult<UserModel> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    let res = conn
        .interact(move |conn| {
            diesel::update(
                users::table
                    .filter(users::id.eq(user_id))
            )
            .set(users::is_active.eq(true))
            .returning(UserDB::as_returning())
            .get_result(conn)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(RepoError::Diesel)?;

    Ok(res.into())
}
