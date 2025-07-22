mod helpers;
use std::sync::Arc;
use std::time::Duration;
use rpc_proxy_meldsun0::services::health_checker::{HealthChecker};
use helpers::create_dummy_blockchain;
use crate::helpers::MockQuerier;

#[tokio::test]
async fn test_health_checker() {

    let blockchain = create_dummy_blockchain();
    let blockchain = Arc::new(blockchain);


    let checker = HealthChecker::new(1, 2, MockQuerier);

    checker.init_health_check_scheduler(Arc::clone(&blockchain));

    tokio::time::sleep(Duration::from_secs(3)).await;

    let provider = &blockchain.providers()[0];
    let health = blockchain.get_provider_health(provider.id()).unwrap();
    assert!(*health.latency() > 0, "Latency should be > 0");
    assert!(*health.last_block() > 0, "Last block should be > 0");
}


