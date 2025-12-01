use std::sync::Arc;
use sqlx::PgPool;
use redis::aio::MultiplexedConnection;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub redis: Arc<MultiplexedConnection>,
    pub config: Arc<AppConfig>,
}


pub struct AppConfig {
    pub database_url: String,
    pub redis_url: String,
    pub iss_api_key: String,
    pub nasa_api_key: String,
    pub osdr_api_key: String,
    pub jwst_api_key: String,
    pub astronomy_api_key: String,
    pub fetch_interval_iss: u64,
    pub fetch_interval_osdr: u64,
    pub fetch_interval_jwst: u64,
    pub fetch_interval_astronomy: u64,
}

impl AppConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgresql://cassiopeia:cassiopeia_pass@iss_db:5432/iss_data".to_string()),
            redis_url: std::env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://redis:6379".to_string()),
            iss_api_key: std::env::var("ISS_API_KEY").unwrap_or_else(|_| "demo_key".to_string()),
            nasa_api_key: std::env::var("NASA_API_KEY").unwrap_or_else(|_| "demo_key".to_string()),
            osdr_api_key: std::env::var("OSDR_API_KEY").unwrap_or_else(|_| "demo_key".to_string()),
            jwst_api_key: std::env::var("JWST_API_KEY").unwrap_or_else(|_| "demo_key".to_string()),
            astronomy_api_key: std::env::var("ASTRONOMY_API_KEY").unwrap_or_else(|_| "demo_key".to_string()),
            fetch_interval_iss: std::env::var("FETCH_INTERVAL_ISS")
                .unwrap_or_else(|_| "300".to_string())
                .parse()
                .unwrap_or(300),
            fetch_interval_osdr: std::env::var("FETCH_INTERVAL_OSDR")
                .unwrap_or_else(|_| "600".to_string())
                .parse()
                .unwrap_or(600),
            fetch_interval_jwst: std::env::var("FETCH_INTERVAL_JWST")
                .unwrap_or_else(|_| "900".to_string())
                .parse()
                .unwrap_or(900),
            fetch_interval_astronomy: std::env::var("FETCH_INTERVAL_ASTRONOMY")
                .unwrap_or_else(|_| "1200".to_string())
                .parse()
                .unwrap_or(1200),
        })
    }
}

