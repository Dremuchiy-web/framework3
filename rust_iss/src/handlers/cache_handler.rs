use axum::{extract::State, Json};
use crate::config::AppState;
use crate::handlers::error::{ApiError, ApiResult};
use crate::repo::cache_repo::CacheRepo;
use serde_json::json;

pub async fn get_cache(
    State(state): State<std::sync::Arc<AppState>>,
    axum::extract::Path(key): axum::extract::Path<String>,
) -> ApiResult<serde_json::Value> {
    let repo = CacheRepo::new(state.redis.clone());
    let value: Option<String> = repo.get(&key).await.map_err(ApiError::Internal)?;
    
    match value {
        Some(v) => Ok(Json(json!({ "key": key, "value": v }))),
        None => Err(ApiError::NotFound),
    }
}

pub async fn set_cache(
    State(state): State<std::sync::Arc<AppState>>,
    axum::extract::Path(key): axum::extract::Path<String>,
    axum::extract::Json(payload): axum::extract::Json<serde_json::Value>,
) -> ApiResult<serde_json::Value> {
    let repo = CacheRepo::new(state.redis.clone());
    repo.set(&key, &payload, 3600).await.map_err(ApiError::Internal)?;
    Ok(Json(json!({ "key": key, "status": "set" })))
}

