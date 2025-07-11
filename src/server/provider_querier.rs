use std::sync::Arc;
use std::time::Duration;
use getset::Getters;
use reqwest::Client;
use serde_json::Value;
use log::error;
use crate::model::blockchain::Blockchain;

pub struct ProviderQuerier {
    client: Client,
}

#[derive(Debug, Clone, Getters)]
pub struct ProviderResponse {
   #[get = "pub"]
   latency_ms: u64,
   #[get = "pub"]
   last_block: u64
}

impl ProviderQuerier {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[cfg(not(feature = "mock"))]
    pub async fn query(&self, url: &str) -> Option<ProviderResponse> {
        let start = tokio::time::Instant::now();
        match self.client.get(url).send().await {
            Ok(response) => {
                let latency = start.elapsed().as_millis() as u64;

                match response.json::<Value>().await {
                    Ok(json) => {
                        if let Some(last_block) = json["last_block"].as_u64() {
                            Some(ProviderResponse {
                                latency_ms: latency,
                                last_block,
                            })
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


    #[cfg(feature = "mock")]
    pub async fn query(&self, url: &str) -> Option<ProviderResponse> {
        use rand::Rng;
        use tokio::time::sleep;
        use std::time::Duration;

        let latency = rand::thread_rng().gen_range(50..250);
        let last_block = rand::thread_rng().gen_range(10_000_000..20_000_000);

        sleep(Duration::from_millis(latency)).await;

        println!(
            "Mocked query: url={} latency={}ms last_block={}",
            url, latency, last_block
        );

        Some(ProviderResponse {
            latency_ms: latency,
            last_block,
        })
    }

}
