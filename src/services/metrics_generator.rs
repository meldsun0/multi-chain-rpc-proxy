use std::panic;
use log::error;
use crate::model::blockchain::Blockchain;
use crate::model::metrics::{BlockchainMetrics, HealthMetric, ProviderMetrics, RankedProviderMetric};
use crate::model::provider::Provider;
use crate::model::provider_health::Health;
use crate::model::provider_ranker::RankedProvider;

pub struct MetricGenerator;
impl MetricGenerator {
    fn generate_provider_metrics(provider: &Provider, health: Option<Health>, ranked: Option<&RankedProvider>) -> Result<ProviderMetrics, ()> {
        panic::catch_unwind(|| {
            let health_data = health.as_ref().map(HealthMetric::from);
            let rank = ranked.map(RankedProviderMetric::from);

            ProviderMetrics::new(
                provider.id().to_string(),
                provider.name().to_string(),
                *provider.priority(),
                *provider.default(),
                health_data,
                rank,
            )
        }).map_err(|_| {
            error!("Panic occurred while generating metrics for provider '{}'",provider.id());
        })
    }

    pub fn generate_metrics(blockchain: &Blockchain) -> Result<BlockchainMetrics, ()> {
        let rankings = blockchain.get_rankings();

        let providers: Vec<ProviderMetrics> = blockchain
            .providers()
            .iter()
            .map(|provider| {
                let health = blockchain.get_provider_health(provider.id());
                let rank = rankings.iter().find(|r| r.id() == provider.id());

                Self::generate_provider_metrics(provider, health, rank)
            })
            .collect::<Result<_, _>>()?;

        Ok(BlockchainMetrics::new(
            blockchain.id().to_string(),
            blockchain.name().to_string(),
            providers,
        ))
    }
}
