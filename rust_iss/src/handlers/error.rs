use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Validation error: {0}")]
    Validation(#[from] validator::ValidationErrors),
    
    #[error("HTTP client error: {0}")]
    Http(#[from] reqwest::Error),
    
    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Not found")]
    NotFound,
    
    #[error("Internal server error: {0}")]
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApiError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
            ApiError::Validation(_) => (StatusCode::BAD_REQUEST, "Validation error"),
            ApiError::Http(_) => (StatusCode::BAD_GATEWAY, "External API error"),
            ApiError::Redis(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Cache error"),
            ApiError::Serialization(_) => (StatusCode::BAD_REQUEST, "Serialization error"),
            ApiError::NotFound => (StatusCode::NOT_FOUND, "Resource not found"),
            ApiError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
        };

        let body = Json(json!({
            "error": error_message,
            "message": self.to_string(),
        }));

        (status, body).into_response()
    }
}

pub type ApiResult<T> = Result<Json<T>, ApiError>;

