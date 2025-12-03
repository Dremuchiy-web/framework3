use crate::clients::astronomy_client::AstronomyClient;
use crate::domain::astronomy::{AstronomyCreateRequest, AstronomyData};
use crate::domain::validation::astronomy_validator::AstronomyDataValidator;
use crate::repo::astronomy_repo::AstronomyRepo;
use anyhow::Result;

pub struct AstronomyService {
    repo: AstronomyRepo,
    client: AstronomyClient,
}

impl AstronomyService {
    pub fn new(repo: AstronomyRepo, client: AstronomyClient) -> Self {
        Self { repo, client }
    }

    pub async fn fetch_and_save(&self) -> Result<Vec<AstronomyData>> {
        let api_response = self.client.get_objects_with_retry(3).await?;
        
        let mut saved_records = Vec::new();
        
        for object in api_response.objects {
            let observation_date = chrono::DateTime::parse_from_rfc3339(&object.observation_date)
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now());

            let create_request = AstronomyCreateRequest {
                object_name: object.name,
                object_type: object.object_type,
                ra: object.ra,
                dec: object.dec,
                magnitude: object.magnitude,
                distance_ly: object.distance_ly,
                observation_date,
            };

            let validator = AstronomyDataValidator::from_request(&create_request);
            validator::Validate::validate(&validator).map_err(|e| anyhow::anyhow!("Validation error: {:?}", e))?;

            let saved = self.repo.upsert(&create_request).await?;
            saved_records.push(saved);
        }

        Ok(saved_records)
    }

    pub async fn get_all(&self) -> Result<Vec<AstronomyData>> {
        self.repo.get_all().await
    }
}

