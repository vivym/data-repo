use axum::{routing::{get, post, delete}, Router};

use crate::server::AppState;

pub mod create;
pub mod delete;
pub mod error;
pub mod get;
pub mod list;
pub mod schema;

pub fn users_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", post(create::create_permission))
        .route("/", get(list::list_permissions))
        .route("/:id", get(get::get_permission))
        .route("/:id", delete(delete::delete_permission))
        .with_state(state)
}
