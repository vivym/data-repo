use axum::{routing::{get, post, put, delete}, Router};

use crate::server::AppState;

pub mod activate;
pub mod create;
pub mod delete;
pub mod error;
pub mod get;
pub mod group;
pub mod list;
pub mod permission;
pub mod schema;
pub mod update;

pub fn users_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", post(create::create_user))
        .route("/", get(list::list_users))
        .route("/:id", get(get::get_user))
        .route("/:id", put(update::update_user))
        .route("/:id", delete(delete::delete_user))
        .route("/:id/activate", get(activate::activate_user))
        .route("/:id/groups", get(group::get_user_groups))
        .route("/:id/permissions", get(permission::get_user_permissions))
        .with_state(state)
}
