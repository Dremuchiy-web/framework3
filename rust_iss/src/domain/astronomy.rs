use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AstronomyData {
    pub id: Uuid,
    pub object_name: String,
    pub object_type: String,
    pub ra: f64,
    pub dec: f64,
    pub magnitude: Option<f64>,
    pub distance_ly: Option<f64>,
    pub observation_date: DateTime<Utc>,
    pub fetched_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AstronomyApiResponse {
    pub objects: Vec<AstronomyObject>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AstronomyObject {
    pub name: String,
    pub object_type: String,
    pub ra: f64,
    pub dec: f64,
    pub magnitude: Option<f64>,
    pub distance_ly: Option<f64>,
    pub observation_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AstronomyCreateRequest {
    pub object_name: String,
    pub object_type: String,
    pub ra: f64,
    pub dec: f64,
    pub magnitude: Option<f64>,
    pub distance_ly: Option<f64>,
    pub observation_date: DateTime<Utc>,
}

