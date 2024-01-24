use axum::{routing::{get, post, put, delete}, Router};

use crate::{middlewares::auth::AuthLayer, server::AppState};

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
        .route(
            "/me",
            get(get::get_me)
                .layer(AuthLayer::new(state.clone(), None)),
        )
        .route(
            "/me",
            put(update::update_me)
                .layer(AuthLayer::new(state.clone(), None)),
        )
        .route(
            "/me/groups",
            get(group::get_me_groups)
                .layer(AuthLayer::new(state.clone(), None)),
        )
        .route(
            "/me/permissions",
            get(permission::get_me_permissions)
                .layer(AuthLayer::new(state.clone(), None)),
        )
        .route(
            "/",
            get(list::list_users)
                .layer(AuthLayer::new(state.clone(), Some("users.read_all".to_string()))),
        )
        .route(
            "/:id",
            get(get::get_user)
                .layer(AuthLayer::new(state.clone(), Some("users.read_all".to_string()))),
        )
        .route(
            "/:id",
            put(update::update_user)
                .layer(AuthLayer::new(state.clone(), Some("users.update_all".to_string()))),
        )
        .route(
            "/",
            delete(delete::delete_users)
                .layer(AuthLayer::new(state.clone(), Some("users.delete_all".to_string()))),
        )
        .route(
            "/:id",
            delete(delete::delete_user)
                .layer(AuthLayer::new(state.clone(), Some("users.delete_all".to_string()))),
        )
        .route(
            "/:id/activate",
            get(activate::activate_user)
                .layer(AuthLayer::new(state.clone(), Some("users.activate".to_string()))),
        )
        .route(
            "/:id/deactivate",
            get(activate::deactivate_user)
                .layer(AuthLayer::new(state.clone(), Some("users.activate".to_string()))),
        )
        .route(
            "/:id/groups",
            get(group::get_user_groups)
                .layer(AuthLayer::new(state.clone(), Some("users.read_all".to_string()))),
        )
        .route(
            "/:id/permissions",
            get(permission::get_user_permissions)
                .layer(AuthLayer::new(state.clone(), Some("users.read_all".to_string()))),
        )
        .with_state(state)
}
