use axum::{extract::State, Json};
use crate::config::AppState;
use crate::handlers::error::{ApiError, ApiResult};
use crate::services::astronomy_service::AstronomyService;
use crate::clients::astronomy_client::AstronomyClient;
use crate::repo::astronomy_repo::AstronomyRepo;
use serde_json::json;

pub async fn get_all_astronomy_data(
    State(state): State<std::sync::Arc<AppState>>,
) -> ApiResult<serde_json::Value> {
    let repo = AstronomyRepo::new(state.pool.clone());
    let client = AstronomyClient::new(
        "https://api.astronomyapi.com".to_string(),
        state.config.astronomy_api_key.clone(),
    );
    let service = AstronomyService::new(repo, client);
    
    let data = service.get_all().await.map_err(ApiError::Internal)?;
    Ok(Json(json!(data)))
}

pub async fn fetch_astronomy_data(
    State(state): State<std::sync::Arc<AppState>>,
) -> ApiResult<serde_json::Value> {
    let repo = AstronomyRepo::new(state.pool.clone());
    let client = AstronomyClient::new(
        "https://api.astronomyapi.com".to_string(),
        state.config.astronomy_api_key.clone(),
    );
    let service = AstronomyService::new(repo, client);
    
    let data = service.fetch_and_save().await.map_err(ApiError::Internal)?;
    Ok(Json(json!(data)))
}

