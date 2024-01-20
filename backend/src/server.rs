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
    datasets::datasets_routes,
    ds_shard_ver_orders::ds_shard_ver_orders_routes,
    ds_shards::ds_shards_routes,
    users::users_routes,
    auth::{login::login, logout::logout},
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
            // Dataset
            crate::routes::datasets::create_dataset,
            crate::routes::datasets::list_datasets,
            crate::routes::datasets::get_dataset,
            crate::routes::datasets::delete_dataset,
            // DatasetShardVerificationOrder
            crate::routes::ds_shard_ver_orders::create_order,
            crate::routes::ds_shard_ver_orders::list_orders,
            crate::routes::ds_shard_ver_orders::get_order,
            crate::routes::ds_shard_ver_orders::delete_order,
            // DatasetShard
            crate::routes::ds_shards::create_ds_shard,
            crate::routes::ds_shards::list_ds_shards,
            crate::routes::ds_shards::get_ds_shard,
            crate::routes::ds_shards::delete_ds_shard,
            // Users
            crate::routes::users::list::list_users,
            crate::routes::users::get::get_user,
            crate::routes::users::delete::delete_user,
            // Login
            crate::routes::auth::login::login,
            // Logout
            crate::routes::auth::logout::logout,
        ),
        components(
            schemas(
                // Dataset
                crate::routes::datasets::DatasetCreationRequest,
                crate::routes::datasets::DatasetResponse,
                crate::routes::datasets::ListDatasetsResponse,
                crate::routes::datasets::DeleteDatasetResponse,
                // DatasetShardVerificationOrder
                crate::routes::ds_shard_ver_orders::OrderCreationRequest,
                crate::routes::ds_shard_ver_orders::OrderResponse,
                crate::routes::ds_shard_ver_orders::ListOrdersResponse,
                crate::routes::ds_shard_ver_orders::DeleteOrderResponse,
                // DatasetShard
                crate::routes::ds_shards::DatasetShardCreationRequest,
                crate::routes::ds_shards::DatasetShardResponse,
                crate::routes::ds_shards::ListDatasetShardsResponse,
                crate::routes::ds_shards::DeleteDatasetShardResponse,
                // Users
                crate::routes::users::schema::UserSchema,
                crate::routes::users::create::UserCreationRequest,
                crate::routes::users::create::UserCreationResponse,
                crate::routes::users::list::ListUsersResponse,
                crate::routes::users::get::GetUserResponse,
                crate::routes::users::update::UserUpdateRequest,
                crate::routes::users::update::UserUpdateResponse,
                crate::routes::users::delete::DeleteUserResponse,
                // Login
                crate::routes::auth::login::LoginRequest,
                crate::routes::auth::login::LoginResponse,
                // Logout
                crate::routes::auth::logout::LogoutResponse,
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
        .nest("/v1/datasets", datasets_routes(state.clone()))
        .nest("/v1/ds_shard_ver_orders", ds_shard_ver_orders_routes(state.clone()))
        .nest("/v1/ds_shards", ds_shards_routes(state.clone()))
        .nest("/v1/users", users_routes(state.clone()))
        .route("/v1/login", post(login))
        .route("/v1/logout", get(logout))
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
