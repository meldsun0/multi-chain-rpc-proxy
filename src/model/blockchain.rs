use std::sync::Arc;
use getset::{Getters, Setters};
use crate::model::provider::Provider;
use crate::model::provider_health::Health;
use dashmap::DashMap;
#[derive(Debug, Clone, Getters, Setters)]
pub struct Blockchain {
    #[get = "pub"]
    id: String,
    #[get = "pub"]
    name: String,
    #[get = "pub"]
    providers: Vec<Provider>,
    health_map: Arc<DashMap<String, Health>>,

}

impl Blockchain {
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            providers: Vec::new(),
            health_map: Arc::new(DashMap::new()),
        }
    }

    pub fn add_provider(&mut self, provider: Provider) {
        self.providers.push(provider);
    }

    pub fn update_provider_health(&self, provider: &Provider, latency: u64, last_block: u64) {
        let health = Health::new(latency, last_block);
        self.health_map.insert(provider.id().to_string(), health);
    }

    pub fn get_provider_health(&self, provider_id: &str) -> Option<Health> {
        self.health_map.get(provider_id).map(|entry| entry.clone())
    }

}
