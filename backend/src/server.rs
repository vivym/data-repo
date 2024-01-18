use std::net::SocketAddr;

use axum::{Router, http, routing::get, response::IntoResponse};
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
};

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
    dataset_url: String,
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
        dataset_url, deadpool_diesel::Runtime::Tokio1
    );
    let pg_pool = Pool::builder(manager).build()?;
    run_migrations(&pg_pool).await;

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
        .layer(OtelAxumLayer::default())
        .layer(cors_layer)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(addr).await?;
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
