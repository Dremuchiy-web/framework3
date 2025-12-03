use axum::{
    routing::{get, post},
    Router,
};
use crate::config::AppState;
use std::sync::Arc;
use crate::handlers::iss_handler;

pub fn create_iss_routes() -> Router<()> {
    Router::new()
        .route("/api/iss", get(iss_handler::get_all_iss_data))
        .route("/api/iss/latest", get(iss_handler::get_latest_iss_data))
        .route("/api/iss/fetch", post(iss_handler::fetch_iss_data))
}

