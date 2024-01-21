use axum::{routing::{get, post, put, delete}, Router};

use crate::{middlewares::auth::AuthLayer, server::AppState};

pub mod create;
pub mod delete;
pub mod error;
pub mod get;
pub mod list;
pub mod schema;
pub mod update;

pub fn ds_shards_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/",
            post(create::create_dataset_shard)
                .layer(AuthLayer::new(state.clone(), Some("datasets.shards.create".to_string()))),
        )
        .route(
            "/",
            get(list::list_dataset_shards)
                .layer(AuthLayer::new(state.clone(), Some("datasets.shards.read".to_string()))),
        )
        .route(
            "/:id",
            get(get::get_dataset_shard)
                .layer(AuthLayer::new(state.clone(), Some("datasets.shards.read".to_string()))),
        )
        .route(
            "/:id",
            put(update::update_dataset_shard)
                .layer(AuthLayer::new(state.clone(), Some("datasets.shards.update".to_string()))),
        )
        .route(
            "/:id",
            delete(delete::delete_dataset_shard)
                .layer(AuthLayer::new(state.clone(), Some("datasets.shards.delete".to_string()))),
        )
        .with_state(state)
}
