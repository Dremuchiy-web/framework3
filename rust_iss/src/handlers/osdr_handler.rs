use axum::{extract::State, Json};
use crate::config::AppState;
use crate::handlers::error::{ApiError, ApiResult};
use crate::services::osdr_service::OsdrService;
use crate::clients::osdr_client::OsdrClient;
use crate::repo::osdr_repo::OsdrRepo;
use serde_json::json;

pub async fn get_all_osdr_data(
    State(state): State<std::sync::Arc<AppState>>,
) -> ApiResult<serde_json::Value> {
    let repo = OsdrRepo::new(state.pool.clone());
    let client = OsdrClient::new(
        "https://api.osdr.nasa.gov".to_string(),
        state.config.osdr_api_key.clone(),
    );
    let service = OsdrService::new(repo, client);
    
    let data = service.get_all().await.map_err(ApiError::Internal)?;
    Ok(Json(json!(data)))
}

pub async fn fetch_osdr_data(
    State(state): State<std::sync::Arc<AppState>>,
) -> ApiResult<serde_json::Value> {
    let repo = OsdrRepo::new(state.pool.clone());
    let client = OsdrClient::new(
        "https://api.osdr.nasa.gov".to_string(),
        state.config.osdr_api_key.clone(),
    );
    let service = OsdrService::new(repo, client);
    
    let data = service.fetch_and_save().await.map_err(ApiError::Internal)?;
    Ok(Json(json!(data)))
}

