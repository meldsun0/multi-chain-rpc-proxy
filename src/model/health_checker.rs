use std::sync::Arc;
use reqwest::Client;
use tokio::time::{interval, Duration, Interval};
use log::{error};
use crate::model::blockchain::Blockchain;

#[derive(Clone)]
pub struct HealthChecker {
    timeout: Duration,
    interval_duration: Duration,
    client: Client,
}

impl HealthChecker {
    pub fn new(timeout: Duration, interval_duration: Duration) -> Self {
        Self {
            timeout,
            interval_duration,
            client: Client::builder()
                .timeout(timeout)
                .build()
                .expect("Failed to build client"),
        }
    }

     async fn check_blockchain_providers(&self, blockchain: Arc<Blockchain>) {
        for provider in blockchain.providers() {
            let url = provider.get_health_endpoint_url();
            let start = tokio::time::Instant::now();

            match self.client.get(&url).send().await {
                Ok(response) => {
                    let latency = start.elapsed().as_millis() as u64;
                    match response.json::<serde_json::Value>().await {
                        Ok(json) => {
                            if let Some(last_block) = json["last_block"].as_u64() {
                                blockchain.update_provider_health(provider, latency, last_block);
                            } else {
                                error!("Provider '{}' at '{}' returned invalid or missing 'last_block': {:?}",provider.id(),url,json);
                            }
                        }
                        Err(e) => {
                            error!("Failed to parse JSON from provider '{}' at '{}': {}",provider.id(),url,e);
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to query provider '{}' at '{}': {}",provider.id(),url,e);
                }
            }
        }
    }


    pub fn init_health_check_scheduler(self: Arc<Self>, blockchain: Arc<Blockchain>) {
        tokio::spawn(async move {
            let mut ticker = tokio::time::interval(self.interval_duration);

            loop {
                ticker.tick().await;
                self.check_blockchain_providers(Arc::clone(&blockchain)).await;
            }
        });
    }
}
