use axum::{
    routing::{get, post},
    Router,
};
use crate::config::AppState;
use std::sync::Arc;
use crate::handlers::jwst_handler;

pub fn create_jwst_routes() -> Router {
    Router::new()
        .route("/api/jwst", get(jwst_handler::get_all_jwst_data))
        .route("/api/jwst/fetch", post(jwst_handler::fetch_jwst_data))
}

