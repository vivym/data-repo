use axum::{routing::{get, post, delete}, Router};

use crate::{middlewares::auth::AuthLayer, server::AppState};

pub mod create;
pub mod delete;
pub mod error;
pub mod get;
pub mod list;
pub mod schema;

pub fn permissions_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/",
            post(create::create_permission)
                .layer(AuthLayer::new(state.clone(), Some("permissions.create".to_string()))),
        )
        .route(
            "/",
            get(list::list_permissions)
                .layer(AuthLayer::new(state.clone(), Some("permissions.read".to_string()))),
        )
        .route(
            "/:id",
            get(get::get_permission)
                .layer(AuthLayer::new(state.clone(), Some("permissions.read".to_string()))),
        )
        .route(
            "/:id",
            delete(delete::delete_permission)
                .layer(AuthLayer::new(state.clone(), Some("permissions.delete".to_string()))),
        )
        .with_state(state)
}
