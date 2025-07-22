use anyhow::Context;
use rpc_proxy_meldsun0::model::provider::Provider;
use rpc_proxy_meldsun0::model::provider_health::Health;
use rpc_proxy_meldsun0::model::provider_ranker::ProviderRanker;
use dashmap::DashMap;

#[test]
fn test_provider_ranker_update_and_get() {
    let provider = Provider::new("provider1", "Provider 1", "localhost", 8545, "/", 1, true).context("Failed to create provider instance");

    let health_map = DashMap::new();
    health_map.insert("provider1".into(),  Health::new(100, 123_456).unwrap());
    let ranker = ProviderRanker::new();
    ranker.update_rankings(&[provider.unwrap().clone()], &health_map);
    let best = ranker.get_best().expect("Should have a best provider");
    assert_eq!(best.id(), "provider1");
    assert_eq!(*best.latency_ms(), 100);
    assert_eq!(*best.last_block(), 123_456);
    assert_eq!(*best.priority(), 1);
    assert_eq!(*best.score(), 12136);
    let all = ranker.get_all();
    assert_eq!(all.len(), 1);
    assert_eq!(all[0].id(), "provider1");
}
