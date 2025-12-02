use crate::domain::iss::{IssData, IssCreateRequest};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;
use anyhow::Result;

pub struct IssRepo {
    pool: PgPool,
}

impl IssRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn upsert(&self, data: &IssCreateRequest) -> Result<IssData> {
        let now = Utc::now();
        
        let record = sqlx::query_as::<_, IssData>(
            r#"
            INSERT INTO iss_data (latitude, longitude, altitude, velocity, visibility, timestamp, fetched_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT (timestamp) 
            DO UPDATE SET 
                latitude = EXCLUDED.latitude,
                longitude = EXCLUDED.longitude,
                altitude = EXCLUDED.altitude,
                velocity = EXCLUDED.velocity,
                visibility = EXCLUDED.visibility,
                updated_at = EXCLUDED.updated_at
            RETURNING *
            "#,
        )
        .bind(data.latitude)
        .bind(data.longitude)
        .bind(data.altitude)
        .bind(data.velocity)
        .bind(&data.visibility)
        .bind(data.timestamp)
        .bind(now)
        .bind(now)
        .fetch_one(&self.pool)
        .await?;

        Ok(record)
    }

    pub async fn get_all(&self) -> Result<Vec<IssData>> {
        let records = sqlx::query_as::<_, IssData>(
            "SELECT * FROM iss_data ORDER BY timestamp DESC"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(records)
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<IssData>> {
        let record = sqlx::query_as::<_, IssData>(
            "SELECT * FROM iss_data WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(record)
    }

    pub async fn get_latest(&self) -> Result<Option<IssData>> {
        let record = sqlx::query_as::<_, IssData>(
            "SELECT * FROM iss_data ORDER BY timestamp DESC LIMIT 1"
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(record)
    }
}

