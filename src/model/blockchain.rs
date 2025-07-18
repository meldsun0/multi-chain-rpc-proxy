use std::sync::Arc;
use getset::{Getters, Setters};
use crate::model::provider::Provider;
use crate::model::provider_health::Health;
use dashmap::DashMap;
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

    pub fn update_provider_health(&self, provider_id: String, latency: u64, last_block: u64) {
        //TODO check provider id is valid.
        let health = Health::new(latency, last_block);
        self.health_map.insert(provider_id, health);
    }

    pub fn get_provider_health(&self, provider_id: &str) -> Option<Health> {
        //TODO check provider id is valid.
        self.health_map.get(provider_id).map(|entry| entry.clone())
    }

    pub fn get_rankings(&self) -> Vec<RankedProvider> {
        self.provider_ranker.get_all()
    }

    pub fn get_best_provider_url(&self) -> Option<String>{
        if let Some(ranked) = self.provider_ranker.get_best() {
            if let Some(provider) = self.get_provider_by_id(&ranked.id()) {
                return Some(provider.get_health_endpoint_url().to_string())
            }
        }
        self.providers
            .iter()
            .find(|p| *p.default())
            .map(|p| p.get_health_endpoint_url().to_string())
    }
    pub fn update_rankings(&self){
        self.provider_ranker.update_rankings(&self.providers, &self.health_map);
    }
    fn get_provider_by_id(&self, id: &str) -> Option<&Provider> {
        //TODO check provider id is valid.
        self.providers.iter().find(|p| p.id() == id)
    }

}

