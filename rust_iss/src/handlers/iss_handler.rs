use axum::{extract::State, Json};
use crate::config::AppState;
use crate::handlers::error::{ApiError, ApiResult};
use crate::services::iss_service::IssService;
use crate::clients::iss_client::IssClient;
use crate::repo::iss_repo::IssRepo;
use serde_json::json;

pub async fn get_all_iss_data(
    State(state): State<std::sync::Arc<AppState>>,
) -> ApiResult<serde_json::Value> {
    let repo = IssRepo::new(state.pool.clone());
    let client = IssClient::new(
        "http://api.open-notify.org".to_string(),
        state.config.iss_api_key.clone(),
    );
    let service = IssService::new(repo, client);
    
    let data = service.get_all().await.map_err(ApiError::Internal)?;
    Ok(Json(json!(data)))
}

pub async fn get_latest_iss_data(
    State(state): State<std::sync::Arc<AppState>>,
) -> ApiResult<serde_json::Value> {
    let repo = IssRepo::new(state.pool.clone());
    let client = IssClient::new(
        "http://api.open-notify.org".to_string(),
        state.config.iss_api_key.clone(),
    );
    let service = IssService::new(repo, client);
    
    let data = service.get_latest().await.map_err(ApiError::Internal)?;
    Ok(Json(json!(data)))
}

pub async fn fetch_iss_data(
    State(state): State<std::sync::Arc<AppState>>,
) -> ApiResult<serde_json::Value> {
    let repo = IssRepo::new(state.pool.clone());
    let client = IssClient::new(
        "http://api.open-notify.org".to_string(),
        state.config.iss_api_key.clone(),
    );
    let service = IssService::new(repo, client);
    
    let data = service.fetch_and_save().await.map_err(ApiError::Internal)?;
    Ok(Json(json!(data)))
}

