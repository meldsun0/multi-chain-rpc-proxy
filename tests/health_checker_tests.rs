mod helpers;
use std::sync::Arc;
use std::time::Duration;
use async_trait::async_trait;
use rpc_proxy_meldsun0::services::health_checker::{HealthChecker, ProviderQuery};
use helpers::create_dummy_blockchain;
use rpc_proxy_meldsun0::services::default_provider_client::ProviderResponse;
use crate::helpers::MockQuerier;

#[tokio::test]
async fn test_health_checker() {

    let blockchain = create_dummy_blockchain();
    let blockchain = Arc::new(blockchain);


    let checker = HealthChecker::new(1, 2, MockQuerier);

    checker.init_health_check_scheduler(Arc::clone(&blockchain));

    tokio::time::sleep(Duration::from_secs(3)).await;

    let provider = &blockchain.providers()[0];
    let health = blockchain.get_provider_health(provider.id());
    assert!(health.is_some());
}


