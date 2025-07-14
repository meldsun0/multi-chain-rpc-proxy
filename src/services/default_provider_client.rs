use std::time::Duration;
use async_trait::async_trait;
use derive_new::new;
use getset::{Getters, Setters};
use log::error;
use reqwest::Client;
use serde_json::Value;
use crate::services::health_checker::ProviderQuery;

#[derive(Clone)]
pub struct DefaultProviderQuerier {
    client: Client,
}

impl DefaultProviderQuerier {
    pub fn new(timeout: u64) -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(timeout))
                .build()
                .expect("Failed to build client")
        }
    }
}

#[derive(Debug, Clone, new, Getters)]
pub struct ProviderResponse {
   #[get = "pub"]
   latency_ms: u64,
   #[get = "pub"]
   last_block: u64
}

#[async_trait]
impl ProviderQuery for DefaultProviderQuerier {
    async fn query(&self, url: &str) -> Option<ProviderResponse> {
        let start = tokio::time::Instant::now();
        match self.client.get(url).send().await {
            Ok(response) => {
                let latency = start.elapsed().as_millis() as u64;

                match response.json::<Value>().await {
                    Ok(json) => {
                        if let Some(last_block) = json["last_block"].as_u64() {
                            Some(ProviderResponse::new(latency, last_block))
                        } else {
                            error!("Invalid or missing 'last_block' at '{}': {:?}", url, json);
                            None
                        }
                    }
                    Err(e) => {
                        error!("Failed to parse JSON from '{}': {}", url, e);
                        None
                    }
                }
            }
            Err(e) => {
                error!("Failed to query '{}': {}", url, e);
                None
            }
        }
    }
}


