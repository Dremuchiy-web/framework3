use crate::clients::iss_client::IssClient;
use crate::domain::iss::{IssCreateRequest, IssData};
use crate::domain::validation::iss_validator::IssDataValidator;
use crate::repo::iss_repo::IssRepo;
use anyhow::Result;
use chrono::{DateTime, Utc};

pub struct IssService {
    repo: IssRepo,
    client: IssClient,
}

impl IssService {
    pub fn new(repo: IssRepo, client: IssClient) -> Self {
        Self { repo, client }
    }

    pub async fn fetch_and_save(&self) -> Result<IssData> {
        let api_response = self.client.get_position_with_retry(3).await?;
        
        let timestamp = DateTime::from_timestamp(api_response.timestamp, 0)
            .unwrap_or_else(Utc::now);

        let create_request = IssCreateRequest {
            latitude: api_response.iss_position.latitude.parse()?,
            longitude: api_response.iss_position.longitude.parse()?,
            altitude: 408.0,
            velocity: 27600.0,
            visibility: "daylight".to_string(),
            timestamp,
        };

        let validator = IssDataValidator::from_request(&create_request);
        validator::Validate::validate(&validator).map_err(|e| anyhow::anyhow!("Validation error: {:?}", e))?;

        let saved = self.repo.upsert(&create_request).await?;
        Ok(saved)
    }

    pub async fn get_all(&self) -> Result<Vec<IssData>> {
        self.repo.get_all().await
    }

    pub async fn get_latest(&self) -> Result<Option<IssData>> {
        self.repo.get_latest().await
    }
}

