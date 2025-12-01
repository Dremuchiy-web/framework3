use axum::Router;
use std::sync::Arc;
use sqlx::PgPool;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::timeout::TimeoutLayer;
use tracing_subscriber;

mod config;
mod domain;
mod handlers;
mod services;
mod clients;
mod repo;

use config::AppConfig;
use config::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let config = AppConfig::from_env()?;
    
    let pool = PgPool::connect(&config.database_url).await?;
    
    sqlx::migrate!("./migrations").run(&pool).await?;

    let redis_client = redis::Client::open(config.redis_url.as_str())?;
    let redis_conn = redis_client.get_multiplexed_async_connection().await?;

    let app_state = Arc::new(AppState {
        pool,
        redis: Arc::new(redis_conn),
        config: Arc::new(config),
    });

    let app = create_router(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("Server listening on http://0.0.0.0:3000");
    
    // В axum 0.7 Router с состоянием нужно преобразовать правильно
    // Используем стандартный метод для запуска сервера
    axum::serve(listener, app).await?;

    Ok(())
}

fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/iss", axum::routing::get(crate::handlers::iss_handler::get_all_iss_data))
        .route("/api/iss/latest", axum::routing::get(crate::handlers::iss_handler::get_latest_iss_data))
        .route("/api/iss/fetch", axum::routing::post(crate::handlers::iss_handler::fetch_iss_data))
        .route("/api/osdr", axum::routing::get(crate::handlers::osdr_handler::get_all_osdr_data))
        .route("/api/osdr/fetch", axum::routing::post(crate::handlers::osdr_handler::fetch_osdr_data))
        .route("/api/jwst", axum::routing::get(crate::handlers::jwst_handler::get_all_jwst_data))
        .route("/api/jwst/fetch", axum::routing::post(crate::handlers::jwst_handler::fetch_jwst_data))
        .route("/api/astronomy", axum::routing::get(crate::handlers::astronomy_handler::get_all_astronomy_data))
        .route("/api/astronomy/fetch", axum::routing::post(crate::handlers::astronomy_handler::fetch_astronomy_data))
        .route("/api/cache/:key", axum::routing::get(crate::handlers::cache_handler::get_cache))
        .route("/api/cache/:key", axum::routing::post(crate::handlers::cache_handler::set_cache))
        .layer(CorsLayer::permissive())
        .with_state(state)
}

