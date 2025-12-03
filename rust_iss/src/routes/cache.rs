use axum::{
    routing::{get, post},
    Router,
};
use crate::config::AppState;
use std::sync::Arc;
use crate::handlers::cache_handler;

pub fn create_cache_routes() -> Router {
    Router::new()
        .route("/api/cache/:key", get(cache_handler::get_cache))
        .route("/api/cache/:key", post(cache_handler::set_cache))
}

