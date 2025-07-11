pub mod provider;
pub mod blockchain;
pub mod provider_health;
mod health_checker;
pub mod metrics;

use std::panic;
use std::sync::Arc;
use std::time::Duration;
use dashmap::DashMap;
use log::error;
use crate::config::AppConfig;
use crate::model::blockchain::Blockchain;
use crate::model::health_checker::HealthChecker;
use crate::model::metrics::{BlockchainMetrics, HealthMetric, Metrics, ProviderMetrics};
use crate::model::provider::Provider;

#[derive(Default)]
pub struct AppState {
    chains: DashMap<String, Arc<Blockchain>>,

}

impl AppState {
    pub fn new(config: &AppConfig) -> Self {
        let chains = DashMap::new();

        for bc in &config.blockchains {
            let mut blockchain = Blockchain::new(bc.id.clone(), bc.name.clone());

            for p in &bc.providers {
                let provider = Provider::new(p.id.clone(), p.name.clone(), p.host.clone(), p.port, p.path.clone(), p.priority, p.default);
                blockchain.add_provider(provider);
            }
            chains.insert(bc.id.clone(), Arc::new(blockchain));
        }
        AppState { chains }
    }

    pub fn add_blockchain(&self, chain_id: String, blockchain: Blockchain) {
        self.chains.insert(chain_id, Arc::new(blockchain));
    }

    pub fn get_blockchain(&self, chain_id: &str) -> Option<Arc<Blockchain>> {
        self.chains.get(chain_id).map(|entry| Arc::clone(&entry))
    }

    pub fn init_health_checker(&self) {
        for entry in self.chains.iter() {
            let blockchain = Arc::clone(entry.value());

            let checker = Arc::new(HealthChecker::new(
                Duration::from_secs(3),
                Duration::from_secs(30),
            ));

            checker.init_health_check_scheduler(blockchain);
        }
    }
    pub fn get_best_provider_url(&self, chain_id: &str) -> Option<String> {
        match self.chains.get(chain_id) {
            Some(entry) => entry.get_best_provider_url(),
            None => {
                error!("No blockchain found for chain_id '{}'", chain_id);
                None
            }
        }
    }
    pub fn get_metrics(&self) -> Option<Metrics> {
        let blockchains_result: Result<Vec<BlockchainMetrics>, ()> = self.chains.iter().map(|entry| {
            let blockchain = entry.value();
            let rankings = blockchain.get_rankings();

            let providers_result: Result<Vec<ProviderMetrics>, ()> = blockchain
                .providers()
                .iter()
                .map(|provider| {
                    let health = blockchain.get_provider_health(provider.id());
                    let rank = rankings.iter().find(|r| r.id == *provider.id());

                    ProviderMetrics::generate_metrics(provider, health, rank)
                        .map_err(|_| error!("Failed to generate metrics for provider '{}'", provider.id()))
                })
                .collect();

            BlockchainMetrics::generate_metrics(blockchain.id(), blockchain.name(), providers_result)
        }).collect();

        match blockchains_result {
            Ok(blockchains) => Some(Metrics { blockchains }),
            Err(_) => {
                error!("Failed to build metrics: one or more blockchain metrics could not be generated.");
                None
            }
        }
    }
}