use std::sync::Arc;
use anyhow::Context;
use getset::{Getters, Setters};
use crate::model::provider::Provider;
use crate::model::provider_health::Health;
use dashmap::DashMap;
use thiserror::Error;
use tracing::info;
use crate::model::provider_ranker::{ProviderRanker, RankedProvider};

#[derive(Debug, Clone, Getters, Setters)]
pub struct Blockchain {
    #[get = "pub"]
    id: String,
    #[get = "pub"]
    name: String,
    #[get = "pub"]
    providers: Vec<Provider>,
    health_map: Arc<DashMap<String, Health>>,
    provider_ranker: ProviderRanker,
}

#[derive(Debug, Error)]
pub enum BlockchainError {
    #[error("Unknown provider id: {0}")]
    UnknownProvider(String),

    #[error("Invalid health data for provider '{0}'")]
    InvalidHealthData(String),

    #[error(transparent)]
    HealthConstructionFailed(#[from] anyhow::Error),

}


impl Blockchain {
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            providers: Vec::new(),
            health_map: Arc::new(DashMap::new()),
            provider_ranker: ProviderRanker::new(),
        }
    }

    pub fn add_provider(&mut self, provider: Provider) {
        self.providers.push(provider);
    }

    pub fn update_provider_health(&self, provider_id: String, latency: u64, last_block: u64) -> Result<(), BlockchainError> {
        self.get_provider_by_id(&provider_id)?;
        let health = Health::new(latency, last_block)
            .with_context(|| format!("Failed to create health for provider '{}'", provider_id.clone()))?;
        self.health_map.insert(provider_id, health);
        Ok(())
    }

    pub fn get_provider_health(&self, provider_id: &str) -> Result<Health, BlockchainError> {
        self.get_provider_by_id(provider_id)?;
        self.health_map
            .get(provider_id)
            .map(|entry| entry.clone())
            .ok_or_else(|| BlockchainError::InvalidHealthData(provider_id.to_string()))
    }

    pub fn get_rankings(&self) -> Vec<RankedProvider> {
        self.provider_ranker.get_all()
    }

    pub fn get_best_provider_url(&self) -> Option<String> {
        if let Some(ranked) = self.provider_ranker.get_best() {
            if let Ok(provider) = self.get_provider_by_id(&ranked.id()) {
                return Some(provider.get_health_endpoint_url().to_string());
            }
        }
        self.providers
            .iter()
            .find(|p| *p.default())
            .map(|p| p.get_health_endpoint_url().to_string())
    }
    pub fn update_rankings(&self) {
        info!("Updating rankings for {}", self.name);
        self.provider_ranker.update_rankings(&self.providers, &self.health_map);
    }

    fn get_provider_by_id(&self, id: &str) -> Result<&Provider, BlockchainError> {
        //TODO check provider id is valid.
        self.providers
            .iter()
            .find(|p| p.id() == id)
            .ok_or_else(|| BlockchainError::UnknownProvider(id.to_string()))
    }
}

