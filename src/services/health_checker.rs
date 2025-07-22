use std::sync::Arc;
use log::warn;
use tokio::time::{Duration};
use crate::model::blockchain::Blockchain;
use crate::services::default_provider_client::{ProviderQueryError, ProviderResponse};

#[async_trait::async_trait]
pub trait ProviderQuery: Send + Sync {
    async fn query(&self, url: &str) -> Result<ProviderResponse, ProviderQueryError>;
}

#[derive(Clone)]
pub struct HealthChecker {
    timeout: u64,
    interval_duration: u64,
    querier: Arc<dyn ProviderQuery>
}

impl HealthChecker {
    pub fn new<T>(timeout: u64, interval_duration: u64, querier: T) -> Arc<Self>
    where
        T: ProviderQuery + 'static,
    {
        Arc::new(Self {
            timeout,
            interval_duration,
            querier: Arc::new(querier),
        })
    }

    async fn check_blockchain_providers(&self, blockchain: Arc<Blockchain>) {
        let querier = Arc::clone(&self.querier);
        for provider in blockchain.providers() {
            let url = provider.get_health_endpoint_url();

            match querier.query(&url).await {
                Ok(resp) => {
                    if let Err(e) = blockchain.update_provider_health(
                        provider.id().to_string(),
                        *resp.latency_ms(),
                        *resp.last_block(),
                    ) {
                        warn!("Failed to update health for provider '{}': {:?}",provider.id(),e);
                    }
                }
                Err(e) => {
                    warn!("Failed to query provider '{}': {}",provider.id(),e);
                }
            }
        }
        blockchain.update_rankings();
    }

    pub fn init_health_check_scheduler(self: Arc<Self>, blockchain: Arc<Blockchain>) {
        tokio::spawn(async move {
            let mut ticker = tokio::time::interval(Duration::from_secs(self.interval_duration));

            loop {
                ticker.tick().await;
                self.check_blockchain_providers(Arc::clone(&blockchain)).await;
            }
        });
    }
}