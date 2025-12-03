use crate::clients::osdr_client::OsdrClient;
use crate::domain::osdr::{OsdrCreateRequest, OsdrData};
use crate::domain::validation::osdr_validator::OsdrDataValidator;
use crate::repo::osdr_repo::OsdrRepo;
use anyhow::Result;

pub struct OsdrService {
    repo: OsdrRepo,
    client: OsdrClient,
}

impl OsdrService {
    pub fn new(repo: OsdrRepo, client: OsdrClient) -> Self {
        Self { repo, client }
    }

    pub async fn fetch_and_save(&self) -> Result<Vec<OsdrData>> {
        let api_response = self.client.get_datasets_with_retry(3).await?;
        
        let mut saved_records = Vec::new();
        
        for dataset in api_response.datasets {
            let created_at = chrono::DateTime::parse_from_rfc3339(&dataset.created_at)
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now());

            let create_request = OsdrCreateRequest {
                dataset_id: dataset.id,
                title: dataset.title,
                description: dataset.description,
                file_count: dataset.file_count,
                size_bytes: dataset.size_bytes,
                created_at,
            };

            let validator = OsdrDataValidator::from_request(&create_request);
            validator::Validate::validate(&validator).map_err(|e| anyhow::anyhow!("Validation error: {:?}", e))?;

            let saved = self.repo.upsert(&create_request).await?;
            saved_records.push(saved);
        }

        Ok(saved_records)
    }

    pub async fn get_all(&self) -> Result<Vec<OsdrData>> {
        self.repo.get_all().await
    }
}

