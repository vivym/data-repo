use axum::{routing::{get, post, put, delete}, Router};

use crate::server::AppState;

pub mod create;
pub mod delete;
pub mod error;
pub mod get;
pub mod list;
pub mod schema;
pub mod update;

pub fn ds_shards_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", post(create::create_dataset_shard))
        .route("/", get(list::list_dataset_shards))
        .route("/:id", get(get::get_dataset_shard))
        .route("/:id", put(update::update_dataset_shard))
        .route("/:id", delete(delete::delete_dataset_shard))
        .with_state(state)
}
