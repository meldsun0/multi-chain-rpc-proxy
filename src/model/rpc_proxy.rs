use std::sync::Arc;
use actix_web::web::Query;
use dashmap::DashMap;
use log::error;
use crate::config::Config;
use crate::model::blockchain::Blockchain;
use crate::model::metrics::{BlockchainMetrics, Metrics};
use crate::services::health_checker::{HealthChecker, ProviderQuery};
use crate::services::metrics_generator::MetricGenerator;
use crate::services::default_provider_client::{DefaultProviderQuerier};
use crate::services::request_forwarder::HttpForwarder;

pub struct RPCProxy {
    chains: DashMap<String, Arc<Blockchain>>,
    forwarder: Arc<dyn HttpForwarder>,
}

impl RPCProxy {
    pub fn new(config: &Config,  forwarder: Arc<dyn HttpForwarder>) -> Self {
        let chains = DashMap::new();

        config.blockchains
            .iter()
            .for_each(|bc| {
                chains.insert(bc.id.clone(), Arc::new(Blockchain::from(bc)));
            });

        RPCProxy { chains, forwarder }
    }

    pub fn add_blockchain(&self, chain_id: String, blockchain: Blockchain) {
        self.chains.insert(chain_id, Arc::new(blockchain));
    }

    pub fn get_blockchain(&self, chain_id: &str) -> Option<Arc<Blockchain>> {
        self.chains.get(chain_id).map(|entry| Arc::clone(&entry))
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
    pub fn get_metrics(&self) -> Option<Metrics> {
        let blockchains_result: Result<Vec<BlockchainMetrics>, ()> = self.chains.iter().map(|entry| {
            let blockchain = entry.value();
            MetricGenerator::generate_metrics(blockchain).map_err(|_| {
                error!("Failed to generate metrics for blockchain '{}'",blockchain.id());
                ()
            })
        }).collect();

        match blockchains_result {
            Ok(blockchains) => Some(Metrics::new( blockchains )),
            Err(_) => {
                error!("Failed to build metrics: one or more blockchain metrics could not be generated.");
                None
            }
        }
    }

    pub fn get_forwarder(&self) -> Arc<dyn HttpForwarder> {
        Arc::clone(&self.forwarder)
    }
}