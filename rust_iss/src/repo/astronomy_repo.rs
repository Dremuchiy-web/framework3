use crate::domain::astronomy::{AstronomyData, AstronomyCreateRequest};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;
use anyhow::Result;

pub struct AstronomyRepo {
    pool: PgPool,
}

impl AstronomyRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn upsert(&self, data: &AstronomyCreateRequest) -> Result<AstronomyData> {
        let now = Utc::now();
        
        let record = sqlx::query_as::<_, AstronomyData>(
            r#"
            INSERT INTO astronomy_data (object_name, object_type, ra, dec, magnitude, distance_ly, observation_date, fetched_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (object_name, observation_date) 
            DO UPDATE SET 
                object_type = EXCLUDED.object_type,
                ra = EXCLUDED.ra,
                dec = EXCLUDED.dec,
                magnitude = EXCLUDED.magnitude,
                distance_ly = EXCLUDED.distance_ly,
                updated_at = EXCLUDED.updated_at
            RETURNING *
            "#,
        )
        .bind(&data.object_name)
        .bind(&data.object_type)
        .bind(data.ra)
        .bind(data.dec)
        .bind(data.magnitude)
        .bind(data.distance_ly)
        .bind(data.observation_date)
        .bind(now)
        .bind(now)
        .fetch_one(&self.pool)
        .await?;

        Ok(record)
    }

    pub async fn get_all(&self) -> Result<Vec<AstronomyData>> {
        let records = sqlx::query_as::<_, AstronomyData>(
            "SELECT * FROM astronomy_data ORDER BY observation_date DESC"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(records)
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<AstronomyData>> {
        let record = sqlx::query_as::<_, AstronomyData>(
            "SELECT * FROM astronomy_data WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(record)
    }
}

