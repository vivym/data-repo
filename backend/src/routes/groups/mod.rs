use axum::{routing::{get, post, delete}, Router};

use crate::{middlewares::auth::AuthLayer, server::AppState};

pub mod create;
pub mod delete;
pub mod error;
pub mod get;
pub mod list;
pub mod schema;

pub fn groups_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/",
            post(create::create_group)
                .layer(AuthLayer::new(state.clone(), Some("groups.create".to_string()))),
        )
        .route(
            "/",
            get(list::list_groups)
                .layer(AuthLayer::new(state.clone(), Some("groups.read".to_string()))),
        )
        .route(
            "/:id",
            get(get::get_group)
                .layer(AuthLayer::new(state.clone(), Some("groups.read".to_string()))),
        )
        .route(
            "/",
            delete(delete::delete_groups)
                .layer(AuthLayer::new(state.clone(), Some("groups.delete".to_string()))),
        )
        .route(
            "/:id",
            delete(delete::delete_group)
                .layer(AuthLayer::new(state.clone(), Some("groups.delete".to_string()))),
        )
        .with_state(state)
}
