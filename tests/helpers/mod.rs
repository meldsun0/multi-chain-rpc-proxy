pub mod mock_forwarder;

use async_trait::async_trait;
use rpc_proxy_meldsun0::model::blockchain::Blockchain;
use rpc_proxy_meldsun0::model::provider::Provider;
use rpc_proxy_meldsun0::services::default_provider_client::ProviderResponse;
use rpc_proxy_meldsun0::services::health_checker::ProviderQuery;

pub fn create_dummy_blockchain() -> Blockchain {
    let provider = Provider::new(
        "provider1",
        "Provider 1",
        "localhost",
        8545,
        "/health",
        1,
        true,
    );

    let mut blockchain = Blockchain::new("chain1", "chain1");
    blockchain.add_provider(provider.clone());
    blockchain.update_provider_health(provider.id().to_string(),100, 123456);
    blockchain.update_rankings();
    blockchain
}


#[derive(Clone)]
pub struct MockQuerier;

#[async_trait]
impl ProviderQuery for MockQuerier {
    async fn query(&self, url: &str) -> Option<ProviderResponse> {
        use rand::Rng;
        use tokio::time::sleep;
        use std::time::Duration;

        let latency = rand::thread_rng().gen_range(50..250);
        let last_block = rand::thread_rng().gen_range(10_000_000..20_000_000);

        sleep(Duration::from_millis(latency)).await;
        println!("Mocked query: url={} latency={}ms last_block={}", url, latency, last_block);
        Some(ProviderResponse::new(latency, last_block))
    }
}