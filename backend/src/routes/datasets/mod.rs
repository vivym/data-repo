use axum::{routing::{get, post, put, delete}, Router};

use crate::server::AppState;

pub mod create;
pub mod delete;
pub mod error;
pub mod get;
pub mod items;
pub mod list;
pub mod schema;
pub mod shards;
pub mod update;

pub fn datasets_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", post(create::create_dataset))
        .route("/", get(list::list_datasets))
        .nest("/items", items::ds_items_routes(state.clone()))
        .nest("/shards", shards::ds_shards_routes(state.clone()))
        .route("/:id", get(get::get_dataset))
        .route("/:id", put(update::update_dataset))
        .route("/:id", delete(delete::delete_dataset))
        .with_state(state)
}
