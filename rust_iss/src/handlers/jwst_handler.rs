use axum::{extract::State, Json};
use crate::config::AppState;
use crate::handlers::error::{ApiError, ApiResult};
use crate::services::jwst_service::JwstService;
use crate::clients::jwst_client::JwstClient;
use crate::repo::jwst_repo::JwstRepo;
use serde_json::json;

pub async fn get_all_jwst_data(
    State(state): State<std::sync::Arc<AppState>>,
) -> ApiResult<serde_json::Value> {
    let repo = JwstRepo::new(state.pool.clone());
    let client = JwstClient::new(
        "https://api.jwst.nasa.gov".to_string(),
        state.config.jwst_api_key.clone(),
    );
    let service = JwstService::new(repo, client);
    
    let data = service.get_all().await.map_err(ApiError::Internal)?;
    Ok(Json(json!(data)))
}

pub async fn fetch_jwst_data(
    State(state): State<std::sync::Arc<AppState>>,
) -> ApiResult<serde_json::Value> {
    let repo = JwstRepo::new(state.pool.clone());
    let client = JwstClient::new(
        "https://api.jwst.nasa.gov".to_string(),
        state.config.jwst_api_key.clone(),
    );
    let service = JwstService::new(repo, client);
    
    let data = service.fetch_and_save().await.map_err(ApiError::Internal)?;
    Ok(Json(json!(data)))
}

