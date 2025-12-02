use crate::domain::iss::{IssApiResponse, IssPosition};
use anyhow::Result;
use reqwest::Client;
use std::time::Duration;

pub struct IssClient {
    client: Client,
    base_url: String,
    api_key: String,
    user_agent: String,
}

impl IssClient {
    pub fn new(base_url: String, api_key: String) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("Cassiopeia-ISS-Client/1.0")
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            base_url,
            api_key,
            user_agent: "Cassiopeia-ISS-Client/1.0".to_string(),
        }
    }

    pub async fn get_current_position(&self) -> Result<IssApiResponse> {
        let url = format!("{}/iss-now.json", self.base_url);
        
        let response = self
            .client
            .get(&url)
            .header("User-Agent", &self.user_agent)
            .send()
            .await?;

        let data: IssApiResponse = response.json().await?;
        Ok(data)
    }

    pub async fn get_position_with_retry(&self, max_retries: u32) -> Result<IssApiResponse> {
        let mut last_error = None;
        
        for attempt in 0..max_retries {
            match self.get_current_position().await {
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

