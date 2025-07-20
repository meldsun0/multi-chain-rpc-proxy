use rpc_proxy_meldsun0::model::blockchain::Blockchain;
use rpc_proxy_meldsun0::model::provider::Provider;

fn dummy_provider(id: &str, priority: u8, host: &str, is_default: bool) -> Provider {
    Provider::new(id, format!("{} name", id), host, 8545, "/health", priority, is_default)
}

#[test]
fn test_blockchain_new() {
    let bc = Blockchain::new("eth", "Ethereum");
    assert_eq!(bc.id(), "eth");
    assert_eq!(bc.name(), "Ethereum");
    assert!(bc.get_rankings().is_empty());

}

#[test]
fn test_add_provider() {
    let mut bc = Blockchain::new("eth", "Ethereum");
    let p = dummy_provider("p1", 1, "xyz",false);
    bc.add_provider(p);
    assert_eq!(bc.providers().len(), 1);
    assert_eq!(bc.providers()[0].id(), "p1");
}

#[test]
fn test_update_and_get_provider_health() {
    let mut bc = Blockchain::new("eth", "Ethereum");
    let p = dummy_provider("p1", 10, "xyz",false);
    bc.add_provider(p.clone());

    bc.update_provider_health(p.id().to_string(), 123, 456);

    let health = bc.get_provider_health("p1").unwrap();
    assert_eq!(*health.latency(), 123);
    assert_eq!(*health.last_block(), 456);
}

#[test]
fn test_get_best_provider_url_fallback_to_default() {
    let mut bc = Blockchain::new("eth", "Ethereum");
    let p1 = dummy_provider("p1", 10, "xyz",false);
    let p2 =dummy_provider("p1", 6, "zzz",true);

    bc.add_provider(p2);
    bc.add_provider(p1);

    bc.update_rankings();
    let url = bc.get_best_provider_url().unwrap();
    assert_eq!(url, "https://zzz:8545/health");
}



#[test]
fn test_update_rankings_and_get_rankings() {
    let mut bc = Blockchain::new("eth", "Ethereum");
    let p1 = dummy_provider("p1", 10, "xyz",false);
    let p2 =dummy_provider("p1", 6, "zzz",true);

    bc.add_provider(p1.clone());
    bc.add_provider(p2.clone());

    bc.update_provider_health(p1.id().to_string(), 1, 200);
    bc.update_provider_health(p2.id().to_string(), 10, 200);

    bc.update_rankings();
    let url = bc.get_best_provider_url().unwrap();
    let rankings = bc.get_rankings();
    assert_eq!(url, "https://xyz:8545/health");
    assert_eq!(rankings.len(), 2);
}
