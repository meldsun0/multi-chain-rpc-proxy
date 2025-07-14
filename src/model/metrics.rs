use serde::Serialize;
use derive_new::new;
use getset::Getters;
use crate::model::provider_health::Health;
use crate::model::provider_ranker::{RankedProvider};

#[derive(Serialize, new, Getters)]
pub struct Metrics {
    blockchains: Vec<BlockchainMetrics>,
}

#[derive(Serialize, new, Getters)]
pub struct BlockchainMetrics {
    #[get = "pub"]
    id: String,
    name: String,
    #[get = "pub"]
    providers: Vec<ProviderMetrics>,
}

#[derive(Serialize, new, Getters)]
pub struct ProviderMetrics {
    #[get = "pub"]
    id: String,
    name: String,
    priority: u8,
    default: bool,
    #[get = "pub"]
    health: Option<HealthMetric>,
    #[get = "pub"]
    rank: Option<RankedProviderMetric>,
}
#[derive(Serialize, new)]
pub struct HealthMetric {
    latency_ms: u64,
    last_block: u64,
}

#[derive(Serialize, new)]
pub struct RankedProviderMetric {
    score: u64,
}

impl From<&RankedProvider> for RankedProviderMetric {
    fn from(rp: &RankedProvider) -> Self {
        Self {
            score: *rp.score(),
        }
    }
}

impl From<&Health> for HealthMetric {
    fn from(h: &Health) -> Self {
        Self {
            latency_ms: *h.latency(),
            last_block: *h.last_block(),
        }
    }
}