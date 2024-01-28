use diesel::prelude::*;

use crate::domain::models::group::GroupModel;
use crate::infra::db::schema::{
    users,
    users_groups_rel,
    groups
};
use crate::infra::repositories::{
    error::{RepoError, RepoResult, map_interact_error},
    group::GroupDB,
};

pub async fn get_groups(
    db: &deadpool_diesel::postgres::Pool,
    user_id: i32,
) -> RepoResult<Vec<GroupModel>> {
    let conn = db
        .get()
        .await
        .map_err(RepoError::Pool)?;

    let res = conn
        .interact(move |conn| {
            users::table
                .inner_join(users_groups_rel::table)
                .inner_join(groups::table.on(
                    groups::id.eq(users_groups_rel::group_id)
                ))
                .filter(users::id.eq(user_id))
                .select(GroupDB::as_select())
                .distinct()
                .load::<GroupDB>(conn)
        })
        .await
        .map_err(map_interact_error)?
        .map_err(RepoError::Diesel)?;

    Ok(res.into_iter().map(Into::into).collect())
}

// pub async fn batch_get_groups(
//     db: &deadpool_diesel::postgres::Pool,
//     user_ids: &[i32],
// ) -> RepoResult<Vec<(i32, Vec<GroupModel>)>> {
//     let conn = db
//         .get()
//         .await
//         .map_err(RepoError::Pool)?;

//     let res = conn
//         .interact(move |conn| {
//             users::table
//                 .left_join(users_groups_rel::table)
//                 .left_join(groups::table.on(
//                     groups::id.eq(users_groups_rel::group_id)
//                 ))
//                 .filter(users::id.eq_any(user_ids))
//                 .select((users::id, Option::<GroupDB>::as_select()))
//                 .distinct()
//                 .load::<(i32, Option<GroupDB>)>(conn)
//         })
//         .await
//         .map_err(map_interact_error)?
//         .map_err(RepoError::Diesel)?;
// }
