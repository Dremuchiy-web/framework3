use crate::clients::jwst_client::JwstClient;
use crate::domain::jwst::{JwstCreateRequest, JwstData};
use crate::domain::validation::jwst_validator::JwstDataValidator;
use crate::repo::jwst_repo::JwstRepo;
use anyhow::Result;

pub struct JwstService {
    repo: JwstRepo,
    client: JwstClient,
}

impl JwstService {
    pub fn new(repo: JwstRepo, client: JwstClient) -> Self {
        Self { repo, client }
    }

    pub async fn fetch_and_save(&self) -> Result<Vec<JwstData>> {
        let api_response = self.client.get_observations_with_retry(3).await?;
        
        let mut saved_records = Vec::new();
        
        for observation in api_response.observations {
            let start_time = chrono::DateTime::parse_from_rfc3339(&observation.start_time)
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now());

            let end_time = observation.end_time
                .and_then(|et| chrono::DateTime::parse_from_rfc3339(&et).ok())
                .map(|dt| dt.with_timezone(&chrono::Utc));

            let create_request = JwstCreateRequest {
                observation_id: observation.id,
                target_name: observation.target_name,
                instrument: observation.instrument,
                observation_type: observation.observation_type,
                start_time,
                end_time,
            };

            let validator = JwstDataValidator::from_request(&create_request);
            validator::Validate::validate(&validator).map_err(|e| anyhow::anyhow!("Validation error: {:?}", e))?;

            let saved = self.repo.upsert(&create_request).await?;
            saved_records.push(saved);
        }

        Ok(saved_records)
    }

    pub async fn get_all(&self) -> Result<Vec<JwstData>> {
        self.repo.get_all().await
    }
}

