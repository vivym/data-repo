use diesel::prelude::*;
use serde::Deserialize;

use crate::domain::models::user::UserModel;
use crate::infra::db::schema::users;
use crate::infra::repositories::error::{RepoError, RepoResult, map_interact_error};
use super::schema::UserDB;

#[derive(Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUserDB {
    pub username: String,
    pub hashed_password: String,
    pub nickname: String,
    pub avatar_uri: String,
}

pub async fn create(
    db: &deadpool_diesel::postgres::Pool,
    new_user: NewUserDB,
) -> RepoResult<UserModel> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    let res = conn
        .interact(|conn| {
            diesel::insert_into(users::table)
                .values(new_user)
                .returning(UserDB::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(RepoError::Diesel)?;

    Ok(res.into())
}
