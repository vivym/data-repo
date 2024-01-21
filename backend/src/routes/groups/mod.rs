use axum::{routing::{get, post, delete}, Router};

use crate::server::AppState;

pub mod create;
pub mod delete;
pub mod error;
pub mod get;
pub mod list;
pub mod schema;

pub fn groups_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", post(create::create_group))
        .route("/", get(list::list_groups))
        .route("/:id", get(get::get_group))
        .route("/:id", delete(delete::delete_group))
        .with_state(state)
}
