use crate::domain::astronomy::AstronomyApiResponse;
use anyhow::Result;
use reqwest::Client;
use std::time::Duration;

pub struct AstronomyClient {
    client: Client,
    base_url: String,
    api_key: String,
    user_agent: String,
}

impl AstronomyClient {
    pub fn new(base_url: String, api_key: String) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("Cassiopeia-Astronomy-Client/1.0")
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            base_url,
            api_key,
            user_agent: "Cassiopeia-Astronomy-Client/1.0".to_string(),
        }
    }

    pub async fn get_objects(&self) -> Result<AstronomyApiResponse> {
        let url = format!("{}/objects", self.base_url);
        
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("User-Agent", &self.user_agent)
            .send()
            .await?;

        let data: AstronomyApiResponse = response.json().await?;
        Ok(data)
    }

    pub async fn get_objects_with_retry(&self, max_retries: u32) -> Result<AstronomyApiResponse> {
        let mut last_error = None;
        
        for attempt in 0..max_retries {
            match self.get_objects().await {
                Ok(data) => return Ok(data),
                Err(e) => {
                    last_error = Some(e);
                    if attempt < max_retries - 1 {
                        tokio::time::sleep(Duration::from_secs(2_u64.pow(attempt))).await;
                    }
                }
            }
        }
        
        Err(last_error.unwrap())
    }
}

