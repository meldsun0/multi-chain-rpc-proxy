use std::sync::Arc;
use reqwest::Client;
use tokio::time::{Duration};
use crate::model::blockchain::Blockchain;
use crate::server::provider_querier::ProviderQuerier;

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
        let querier = ProviderQuerier::new(self.client.clone());

        for provider in blockchain.providers() {
            let url = provider.get_health_endpoint_url();

            if let Some(resp) = querier.query(&url).await {
                blockchain.update_provider_health(provider, *resp.last_block(), *resp.latency_ms());
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
