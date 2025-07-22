use std::sync::Arc;
use dashmap::DashMap;
use log::error;
use crate::cache::cache::{InMemoryKeyValueCache, KeyValueCache};
use crate::config::Config;
use crate::model::blockchain::Blockchain;
use crate::model::metrics::{Metrics};
use crate::services::health_checker::{HealthChecker, ProviderQuery};
use crate::services::metrics_generator::MetricGenerator;
use crate::services::request_forwarder::{ForwarderResponse, HttpForwarder};
use anyhow::{Result, Context};
use thiserror::Error;

pub struct RPCProxy {
    chains: DashMap<String, Arc<Blockchain>>,
    forwarder: Arc<dyn HttpForwarder>,
    cache: InMemoryKeyValueCache::<String, ForwarderResponse>,
}


#[derive(Debug, Error)]
pub enum RPCProxyError {
    #[error("Failed to initialize blockchain '{0}'")]
    BlockchainInit(String),

    #[error("Missing blockchain for chain_id '{0}'")]
    MissingChain(String),

    #[error("Failed to initialize RPCProxy")]
    InitError(#[from] anyhow::Error),

    #[error("Failed to generate metrics for blockchain '{0}'")]
    MetricsGeneration(String),
}

impl RPCProxy {
    pub fn init(config: &Config, forwarder: Arc<dyn HttpForwarder>) -> Result<Self, RPCProxyError> {
        Self::new(config, forwarder).map_err(RPCProxyError::InitError)
    }

    fn new(config: &Config,  forwarder: Arc<dyn HttpForwarder>) -> Result<Self>  {
        let chains = DashMap::new();
        let cache = InMemoryKeyValueCache::<String, ForwarderResponse>::new();

        for bc in &config.blockchains {
            let blockchain = Blockchain::try_from(bc)
                .with_context(|| format!("Failed to initialize blockchain '{}'", bc.id))?;
            chains.insert(bc.id.clone(), Arc::new(blockchain));
        }
        Ok(Self { chains, forwarder, cache })
    }

    pub fn add_blockchain(&self, chain_id: String, blockchain: Blockchain) {
        self.chains.insert(chain_id, Arc::new(blockchain));
    }

    pub fn get_blockchain(&self, chain_id: &str)-> Result<Arc<Blockchain>, RPCProxyError> {
        self.chains.get(chain_id)
            .map(|entry| Arc::clone(&entry))
            .ok_or_else(|| RPCProxyError::MissingChain(chain_id.to_string()))
    }

    pub fn init_health_checker<T>(&self, querier: T)
    where
        T: ProviderQuery + Clone + 'static,
    {
        for entry in self.chains.iter() {
            let checker = HealthChecker::new(3, 30, querier.clone());
            checker.init_health_check_scheduler(Arc::clone(entry.value()));
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
    pub fn get_metrics(&self) -> Result<Metrics, RPCProxyError> {
        let blockchains = self.chains.iter()
            .map(|entry| {
                let blockchain = entry.value();
                MetricGenerator::generate_metrics(blockchain)
                    .map_err(|_| RPCProxyError::MetricsGeneration(blockchain.id().to_string()))
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Metrics::new(blockchains))
    }

    pub async fn forward(&self, url: String, body: String) -> ForwarderResponse {
        let key = self.hash_url_and_body(&url, &body);
        if let Some(cached) = self.cache.get(&key) {
            return cached;
        }
        let response = self.forwarder.forward(url, body).await;
        self.cache.set(key.clone(), response.clone(), 10);
        response
    }

     fn hash_url_and_body(&self, url: &str, body: &str) -> String {
        let input = format!("{}|{}", url, body);
        blake3::hash(input.as_bytes()).to_hex().to_string()
    }
}

