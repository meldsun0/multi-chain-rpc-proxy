
mod helpers;
use rpc_proxy_meldsun0::services::metrics_generator::MetricGenerator;
use helpers::create_dummy_blockchain;
#[test]
fn test_generate_metrics_success() {
    let blockchain = create_dummy_blockchain();
    let result = MetricGenerator::generate_metrics(&blockchain);

    assert!(result.is_ok());
    let metrics = result.unwrap();
    assert_eq!(metrics.id(), "chain1");
    assert_eq!(metrics.providers().len(), 1);

    let provider_metric = &metrics.providers()[0];
    assert_eq!(provider_metric.id(), "provider1");
    assert!(provider_metric.health().is_some());
    assert!(provider_metric.rank().is_some());
}