use axum::{
    routing::{get, post},
    Router,
};
use crate::config::AppState;
use std::sync::Arc;
use crate::handlers::astronomy_handler;

pub fn create_astronomy_routes() -> Router {
    Router::new()
        .route("/api/astronomy", get(astronomy_handler::get_all_astronomy_data))
        .route("/api/astronomy/fetch", post(astronomy_handler::fetch_astronomy_data))
}

