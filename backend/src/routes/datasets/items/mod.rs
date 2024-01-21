use axum::{routing::{get, post, put, delete}, Router};

use crate::server::AppState;

pub mod create;
pub mod delete;
pub mod error;
pub mod get;
pub mod list;
pub mod schema;
pub mod update;

pub fn ds_items_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", post(create::create_dataset_item))
        .route("/", get(list::list_dataset_items))
        .route("/:id", get(get::get_dataset_item))
        .route("/:id", put(update::update_dataset_item))
        .route("/:id", delete(delete::delete_dataset_item))
        .with_state(state)
}
