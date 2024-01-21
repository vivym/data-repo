use axum::{routing::{get, post, put, delete}, Router};

use crate::{middlewares::auth::AuthLayer, server::AppState};

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
        .route(
            "/",
            post(create::create_dataset)
                .layer(AuthLayer::new(state.clone(), Some("datasets.create".to_string()))),
        )
        .route(
            "/",
            get(list::list_datasets)
                .layer(AuthLayer::new(state.clone(), Some("datasets.read".to_string()))),
        )
        .nest("/items", items::ds_items_routes(state.clone()))
        .nest("/shards", shards::ds_shards_routes(state.clone()))
        .route(
            "/:id",
            get(get::get_dataset)
                .layer(AuthLayer::new(state.clone(), Some("datasets.read".to_string()))),
        )
        .route(
            "/:id",
            put(update::update_dataset)
                .layer(AuthLayer::new(state.clone(), Some("datasets.update".to_string()))),
        )
        .route(
            "/:id",
            delete(delete::delete_dataset)
                .layer(AuthLayer::new(state.clone(), Some("datasets.delete".to_string()))),
        )
        .with_state(state)
}
