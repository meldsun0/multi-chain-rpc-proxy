use std::time::Duration;
use async_trait::async_trait;
use derive_new::new;
use getset::{Getters};
use log::error;
use reqwest::Client;
use serde_json::Value;
use thiserror::Error;
use crate::services::health_checker::ProviderQuery;

#[derive(Clone)]
pub struct DefaultProviderQuerier {
    client: Client,
}

#[derive(Debug, Error)]
pub enum ProviderQueryError {
    #[error("Failed to perform request to '{url}': {source}")]
    Request {
        url: String,
        #[source]
        source: reqwest::Error,
    },

    #[error("Failed to parse JSON from '{url}': {source}")]
    JsonParse {
        url: String,
        #[source]
        source: reqwest::Error,
    },

    #[error("Missing or invalid 'last_block' field from '{url}'")]
    MissingLastBlock { url: String },
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
    async fn query(&self, url: &str) -> Result<ProviderResponse, ProviderQueryError> {
        let start = tokio::time::Instant::now();
        let response = self.client.get(url).send().await.map_err(|e| {
            ProviderQueryError::Request {
                url: url.to_string(),
                source: e,
            }
        })?;
        let latency = start.elapsed().as_millis() as u64;
        let json = response.json::<Value>().await.map_err(|e| {
            ProviderQueryError::JsonParse {
                url: url.to_string(),
                source: e,
            }
        })?;
        let last_block = json["last_block"]
            .as_u64()
            .ok_or_else(|| ProviderQueryError::MissingLastBlock {
                url: url.to_string(),
            })?;

        Ok(ProviderResponse::new(latency, last_block))
    }
}


