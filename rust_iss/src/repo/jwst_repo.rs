use crate::domain::jwst::{JwstData, JwstCreateRequest};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;
use anyhow::Result;

pub struct JwstRepo {
    pool: PgPool,
}

impl JwstRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn upsert(&self, data: &JwstCreateRequest) -> Result<JwstData> {
        let now = Utc::now();
        
        let record = sqlx::query_as::<_, JwstData>(
            r#"
            INSERT INTO jwst_data (observation_id, target_name, instrument, observation_type, start_time, end_time, fetched_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT (observation_id) 
            DO UPDATE SET 
                target_name = EXCLUDED.target_name,
                instrument = EXCLUDED.instrument,
                observation_type = EXCLUDED.observation_type,
                start_time = EXCLUDED.start_time,
                end_time = EXCLUDED.end_time,
                updated_at = EXCLUDED.updated_at
            RETURNING *
            "#,
        )
        .bind(&data.observation_id)
        .bind(&data.target_name)
        .bind(&data.instrument)
        .bind(&data.observation_type)
        .bind(data.start_time)
        .bind(data.end_time)
        .bind(now)
        .bind(now)
        .fetch_one(&self.pool)
        .await?;

        Ok(record)
    }

    pub async fn get_all(&self) -> Result<Vec<JwstData>> {
        let records = sqlx::query_as::<_, JwstData>(
            "SELECT * FROM jwst_data ORDER BY start_time DESC"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(records)
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<JwstData>> {
        let record = sqlx::query_as::<_, JwstData>(
            "SELECT * FROM jwst_data WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(record)
    }
}

