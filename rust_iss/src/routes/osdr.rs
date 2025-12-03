use axum::{
    routing::{get, post},
    Router,
};
use crate::config::AppState;
use std::sync::Arc;
use crate::handlers::osdr_handler;

pub fn create_osdr_routes() -> Router {
    Router::new()
        .route("/api/osdr", get(osdr_handler::get_all_osdr_data))
        .route("/api/osdr/fetch", post(osdr_handler::fetch_osdr_data))
}

