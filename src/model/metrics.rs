use std::panic;
use log::error;
use serde::Serialize;
use crate::model::provider::Provider;
use crate::model::provider_health::Health;

#[derive(Serialize)]
pub struct ProviderMetrics {
    pub id: String,
    pub name: String,
    pub priority: u8,
    pub default: bool,
    pub health: Option<HealthData>,
}

#[derive(Serialize)]
pub struct HealthData {
    pub latency_ms: u64,
    pub last_block: u64,
    //add last updated value
}

#[derive(Serialize)]
pub struct BlockchainMetrics {
    pub id: String,
    pub name: String,
    pub providers: Vec<ProviderMetrics>,
}

#[derive(Serialize)]
pub struct Metrics {
    pub blockchains: Vec<BlockchainMetrics>,
}

impl ProviderMetrics {
    pub fn generate_metrics(provider: &Provider, health: Option<Health>) -> Result<Self, ()> {
        panic::catch_unwind(|| {
            let health_data = health.map(|h| HealthData {
                latency_ms: *h.latency(),
                last_block: *h.last_block(),
            });

            ProviderMetrics {
                id: provider.id().to_string(),
                name: provider.name().to_string(),
                priority: *provider.priority(),
                default: *provider.default(),
                health: health_data,
            }
        }).map_err(|_| {
                error!("Panic occurred while generating metrics for provider '{}'",provider.id());
        })
    }
}

impl BlockchainMetrics {
    pub fn generate_metrics(id: &str, name: &str, providers_result: Result<Vec<ProviderMetrics>, ()>) -> Result<Self, ()> {
        providers_result
            .map(|providers| BlockchainMetrics {
                id: id.to_string(),
                name: name.to_string(),
                providers,
            })
            .map_err(|_| {
                error!("Failed to collect provider metrics for blockchain '{}'",id);
                ()
            })
    }
}