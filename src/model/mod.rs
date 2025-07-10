mod provider;
mod blockchain;
mod provider_health;

use std::sync::Arc;
use dashmap::DashMap;
use crate::model::blockchain::Blockchain;


#[derive(Default)]
pub struct AppState {
    chains: DashMap<String, Arc<Blockchain>>,



}

impl AppState {

    pub fn new() -> Self {
        Self {
            chains: DashMap::new(),
        }
    }

    pub fn add_blockchain(&self, chain_id: String, blockchain: Blockchain) {
        self.chains.insert(chain_id, Arc::new(blockchain));
    }

    pub fn get_blockchain(&self, chain_id: &str) -> Option<Arc<Blockchain>> {
        self.chains.get(chain_id).map(|entry| Arc::clone(&entry))
    }

    pub fn select_best_provider(&self, chain_id:  &str) -> Option<&String> {
       // self.rpc_urls.get(chain_id)
        None
    }
    pub fn get_metrics(&self) -> Option<String> {
        None
    }
}