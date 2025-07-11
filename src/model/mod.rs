pub mod provider;
pub mod blockchain;
mod provider_health;
mod health_checker;

use std::sync::Arc;
use std::time::Duration;
use dashmap::DashMap;
use crate::config::AppConfig;
use crate::model::blockchain::Blockchain;
use crate::model::health_checker::HealthChecker;
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
                Duration::from_secs(3),     // Timeout per request
                Duration::from_secs(30),    // Interval between checks
            ));

            checker.init_health_check_scheduler(blockchain);
        }
    }
    pub fn select_best_provider(&self, _chain_id:  &str) -> Option<&String> {
       // self.rpc_urls.get(chain_id)
        None
    }
    pub fn get_metrics(&self) -> Option<String> {
        None
    }
}