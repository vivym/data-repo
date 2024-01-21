use std::net::SocketAddr;

use axum::{Router, http, routing::{post, get}, response::IntoResponse};
use axum_login::{
    tower_sessions::{MemoryStore, SessionManagerLayer},
    AuthManagerLayerBuilder,
};
use axum_tracing_opentelemetry::middleware::OtelAxumLayer;
use deadpool_diesel::postgres::{Manager, Pool};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use tower_http::cors::{AllowOrigin, CorsLayer};
use tracing::instrument;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::routes::{
    auth::{login::login, logout::logout},
    datasets::datasets_routes,
    groups::groups_routes,
    permissions::permissions_routes,
    users::users_routes,
};
use crate::services::auth::Backend;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

#[derive(Clone)]
pub struct AppState {
    pub pg_pool: Pool,
}

#[instrument]
pub async fn not_found(uri: http::Uri) -> impl IntoResponse {
    (
        http::StatusCode::NOT_FOUND,
        format!("No route for {}", uri.path()),
    )
}

#[utoipa::path(
    get,
    path = "/ping",
    responses(
        (status = 200, description = "Everything is working fine"),
        (status = 500, description = "Internal server error"),
        (status = 503, description = "Service unavailable"),
    )
)]
#[instrument]
async fn ping() -> &'static str {
    "OK"
}

pub async fn run(
    addr: SocketAddr,
    allow_origin: Option<AllowOrigin>,
    database_url: String,
) -> Result<(), axum::BoxError> {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            ping,
            // login
            crate::routes::auth::login::login,
            // logout
            crate::routes::auth::logout::logout,
            // datasets
            crate::routes::datasets::create::create_dataset,
            crate::routes::datasets::get::get_dataset,
            crate::routes::datasets::list::list_datasets,
            crate::routes::datasets::update::update_dataset,
            crate::routes::datasets::delete::delete_dataset,
            // datasets/items
            crate::routes::datasets::items::create::create_dataset_item,
            crate::routes::datasets::items::get::get_dataset_item,
            crate::routes::datasets::items::list::list_dataset_items,
            crate::routes::datasets::items::update::update_dataset_item,
            crate::routes::datasets::items::delete::delete_dataset_item,
            // datasets/shards
            crate::routes::datasets::shards::create::create_dataset_shard,
            crate::routes::datasets::shards::get::get_dataset_shard,
            crate::routes::datasets::shards::list::list_dataset_shards,
            crate::routes::datasets::shards::update::update_dataset_shard,
            crate::routes::datasets::shards::delete::delete_dataset_shard,
            // groups
            crate::routes::groups::create::create_group,
            crate::routes::groups::get::get_group,
            crate::routes::groups::list::list_groups,
            crate::routes::groups::delete::delete_group,
            // permissions
            crate::routes::permissions::create::create_permission,
            crate::routes::permissions::get::get_permission,
            crate::routes::permissions::list::list_permissions,
            crate::routes::permissions::delete::delete_permission,
            // users
            crate::routes::users::create::create_user,
            crate::routes::users::get::get_user,
            crate::routes::users::list::list_users,
            crate::routes::users::update::update_user,
            crate::routes::users::delete::delete_user,
            // users/groups
            crate::routes::users::group::get_user_groups,
            // users/permissions
            crate::routes::users::permission::get_user_permissions,
        ),
        components(
            schemas(
                // login
                crate::routes::auth::login::LoginRequest,
                crate::routes::auth::login::LoginResponse,
                // logout
                crate::routes::auth::logout::LogoutResponse,
                // datasets
                crate::routes::datasets::schema::DatasetSchema,
                crate::routes::datasets::create::DatasetCreationRequest,
                crate::routes::datasets::create::DatasetCreationResponse,
                crate::routes::datasets::get::GetDatasetResponse,
                crate::routes::datasets::list::ListDatasetsResponse,
                crate::routes::datasets::update::DatasetUpdateRequest,
                crate::routes::datasets::update::DatasetUpdateResponse,
                crate::routes::datasets::delete::DeleteDatasetResponse,
                // datasets/items
                crate::routes::datasets::items::schema::DatasetItemSchema,
                crate::routes::datasets::items::create::DatasetItemCreationRequest,
                crate::routes::datasets::items::create::DatasetItemCreationResponse,
                crate::routes::datasets::items::get::GetDatasetItemResponse,
                crate::routes::datasets::items::list::ListDatasetItemsResponse,
                crate::routes::datasets::items::update::DatasetItemUpdateRequest,
                crate::routes::datasets::items::update::DatasetItemUpdateResponse,
                crate::routes::datasets::items::delete::DeleteDatasetItemResponse,
                // datasets/shards
                crate::routes::datasets::shards::schema::DatasetShardSchema,
                crate::routes::datasets::shards::create::DatasetShardCreationRequest,
                crate::routes::datasets::shards::create::DatasetShardCreationResponse,
                crate::routes::datasets::shards::get::GetDatasetShardResponse,
                crate::routes::datasets::shards::list::ListDatasetShardsResponse,
                crate::routes::datasets::shards::update::DatasetShardUpdateRequest,
                crate::routes::datasets::shards::update::DatasetShardUpdateResponse,
                crate::routes::datasets::shards::delete::DeleteDatasetShardResponse,
                // groups
                crate::routes::groups::schema::GroupSchema,
                crate::routes::groups::create::GroupCreationRequest,
                crate::routes::groups::create::GroupCreationResponse,
                crate::routes::groups::get::GetGroupResponse,
                crate::routes::groups::list::ListGroupsResponse,
                crate::routes::groups::delete::DeleteGroupResponse,
                // permissions
                crate::routes::permissions::schema::PermissionSchema,
                crate::routes::permissions::create::PermissionCreationRequest,
                crate::routes::permissions::create::PermissionCreationResponse,
                crate::routes::permissions::get::GetPermissionResponse,
                crate::routes::permissions::list::ListPermissionsResponse,
                crate::routes::permissions::delete::DeletePermissionResponse,
                // users
                crate::routes::users::schema::UserSchema,
                crate::routes::users::create::UserCreationRequest,
                crate::routes::users::create::UserCreationResponse,
                crate::routes::users::get::GetUserResponse,
                crate::routes::users::list::ListUsersResponse,
                crate::routes::users::update::UserUpdateRequest,
                crate::routes::users::update::UserUpdateResponse,
                crate::routes::users::delete::DeleteUserResponse,
                // users/groups
                crate::routes::users::group::GetUserGroupsResponse,
                // users/permissions
                crate::routes::users::permission::GetUserPermissionsResponse,
            ),
        ),
        tags(
            (name = "Backend API", description = "Data Repo Backend API"),
        )
    )]
    struct ApiDoc;

    // CORS layer
    let allow_origin = allow_origin.unwrap_or(AllowOrigin::any());
    let cors_layer = CorsLayer::new()
        .allow_methods([http::Method::GET, http::Method::POST])
        .allow_headers([http::header::CONTENT_TYPE])
        .allow_origin(allow_origin);

    let manager = Manager::new(
        database_url, deadpool_diesel::Runtime::Tokio1
    );
    let pg_pool = Pool::builder(manager).build()?;
    run_migrations(&pg_pool).await;

    // Session layer.
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store);

    // Auth service.
    let backend = Backend::new(pg_pool.clone());
    let auth_layer = AuthManagerLayerBuilder::new(
        backend, session_layer
    ).build();

    let state = AppState {
        pg_pool,
    };

    let router = Router::new()
        .fallback(not_found)
        .merge(
            SwaggerUi::new("/docs")
                .url("/api-doc/openapi.json", ApiDoc::openapi()),
        )
        .route("/ping", get(ping))
        .route("/v1/login", post(login))
        .route("/v1/logout", get(logout))
        .nest("/v1/datasets", datasets_routes(state.clone()))
        .nest("/v1/groups", groups_routes(state.clone()))
        .nest("/v1/permissions", permissions_routes(state.clone()))
        .nest("/v1/users", users_routes(state.clone()))
        .layer(auth_layer)
        .layer(cors_layer)
        .layer(OtelAxumLayer::default())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("Listening on {}", addr);
    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn run_migrations(pool: &Pool) {
    let conn = pool.get().await.unwrap();
    conn.interact(|conn| conn.run_pending_migrations(MIGRATIONS).map(|_| ()))
        .await
        .unwrap()
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => (),
        _ = terminate => (),
    }

    tracing::info!("Signal received, starting graceful shutdown");
    opentelemetry::global::shutdown_tracer_provider();
}
