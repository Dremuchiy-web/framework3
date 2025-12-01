use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct IssData {
    pub id: Uuid,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
    pub velocity: f64,
    pub visibility: String,
    pub timestamp: DateTime<Utc>,
    pub fetched_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssApiResponse {
    pub message: String,
    pub timestamp: i64,
    pub iss_position: IssPosition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssPosition {
    pub latitude: String,
    pub longitude: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssCreateRequest {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
    pub velocity: f64,
    pub visibility: String,
    pub timestamp: DateTime<Utc>,
}

