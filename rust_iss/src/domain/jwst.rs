use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct JwstData {
    pub id: Uuid,
    pub observation_id: String,
    pub target_name: String,
    pub instrument: String,
    pub observation_type: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub fetched_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwstApiResponse {
    pub observations: Vec<JwstObservation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwstObservation {
    pub id: String,
    pub target_name: String,
    pub instrument: String,
    pub observation_type: String,
    pub start_time: String,
    pub end_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwstCreateRequest {
    pub observation_id: String,
    pub target_name: String,
    pub instrument: String,
    pub observation_type: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
}

