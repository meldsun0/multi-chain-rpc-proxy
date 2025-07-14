use std::cmp::Ordering;
use std::sync::{Arc, RwLock};
use getset::{Getters};
use crate::model::provider::Provider;
use crate::model::provider_health::Health;

#[derive(Debug, Clone, Getters)]
pub struct RankedProvider {
    #[get = "pub"]
    id: String,
    #[get = "pub"]
    score: u64,
    #[get = "pub"]
    latency_ms: u64,
    #[get = "pub"]
    last_block: u64,
    #[get = "pub"]
    priority: u8,
}

#[derive(Clone, Debug)]
pub struct ProviderRanker {
    rankings: Arc<RwLock<Vec<RankedProvider>>>,
}

impl ProviderRanker {
    pub fn new() -> Self {
        Self {
            rankings: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn update_rankings(&self, providers: &[Provider], health_map: &dashmap::DashMap<String, Health>) {
        let mut new_rankings = Vec::with_capacity(providers.len());

        for provider in providers {
            if let Some(health) = health_map.get(provider.id()) {
                let latency = *health.latency();
                let last_block = *health.last_block();
                let priority = *provider.priority();

                let score = Self::calculate_score(latency, last_block, priority);

                new_rankings.push(RankedProvider {
                    id: provider.id().to_string(),
                    score,
                    latency_ms: latency,
                    last_block,
                    priority,
                });
            }
        }

        new_rankings.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(Ordering::Equal)
        });

        if let Ok(mut rankings) = self.rankings.write() {
            *rankings = new_rankings;
        }
    }

    pub fn get_best(&self) -> Option<RankedProvider> {
        self.rankings.read().ok()?.first().cloned()
    }

    pub fn get_all(&self) -> Vec<RankedProvider> {
        self.rankings.read().map(|r| r.clone()).unwrap_or_default()
    }

    fn calculate_score(latency: u64, last_block: u64, priority: u8) -> u64 {
        let latency_score = 1_000_000.0 / (latency as f64 + 1.0);
        let block_score = last_block as f64 * 0.01;
        let priority_score = priority as f64 * 1000.0;
        (latency_score + block_score + priority_score).round() as u64
    }
}