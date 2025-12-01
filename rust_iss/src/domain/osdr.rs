use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OsdrData {
    pub id: Uuid,
    pub dataset_id: String,
    pub title: String,
    pub description: Option<String>,
    pub file_count: i32,
    pub size_bytes: i64,
    pub created_at: DateTime<Utc>,
    pub fetched_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsdrApiResponse {
    pub datasets: Vec<OsdrDataset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsdrDataset {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub file_count: i32,
    pub size_bytes: i64,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsdrCreateRequest {
    pub dataset_id: String,
    pub title: String,
    pub description: Option<String>,
    pub file_count: i32,
    pub size_bytes: i64,
    pub created_at: DateTime<Utc>,
}

