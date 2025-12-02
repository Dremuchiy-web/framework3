use crate::domain::osdr::{OsdrData, OsdrCreateRequest};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;
use anyhow::Result;

pub struct OsdrRepo {
    pool: PgPool,
}

impl OsdrRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn upsert(&self, data: &OsdrCreateRequest) -> Result<OsdrData> {
        let now = Utc::now();
        
        let record = sqlx::query_as::<_, OsdrData>(
            r#"
            INSERT INTO osdr_data (dataset_id, title, description, file_count, size_bytes, created_at, fetched_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT (dataset_id) 
            DO UPDATE SET 
                title = EXCLUDED.title,
                description = EXCLUDED.description,
                file_count = EXCLUDED.file_count,
                size_bytes = EXCLUDED.size_bytes,
                updated_at = EXCLUDED.updated_at
            RETURNING *
            "#,
        )
        .bind(&data.dataset_id)
        .bind(&data.title)
        .bind(&data.description)
        .bind(data.file_count)
        .bind(data.size_bytes)
        .bind(data.created_at)
        .bind(now)
        .bind(now)
        .fetch_one(&self.pool)
        .await?;

        Ok(record)
    }

    pub async fn get_all(&self) -> Result<Vec<OsdrData>> {
        let records = sqlx::query_as::<_, OsdrData>(
            "SELECT * FROM osdr_data ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(records)
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<OsdrData>> {
        let record = sqlx::query_as::<_, OsdrData>(
            "SELECT * FROM osdr_data WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(record)
    }
}

