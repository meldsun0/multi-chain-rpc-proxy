use rpc_proxy_meldsun0::cache::cache::{InMemoryKeyValueCache, KeyValueCache};
use tokio::time::{Duration};

#[test]
fn test_set_and_get() {
    let  cache = InMemoryKeyValueCache::<String, String>::new();
    cache.set("a".into(), "10".into(), 0);
    assert_eq!(cache.get(&"a".to_string()), Some("10".to_string()));
}

#[test]
fn test_contains() {
    let  cache = InMemoryKeyValueCache::<String, String>::new();
    cache.set("key".into(), "42".into(), 0);
    assert!(cache.contains(&"key".to_string()));
    assert!(!cache.contains(&"not_found".to_string()));
}

#[test]
fn test_remove() {
    let cache = InMemoryKeyValueCache::<String, String>::new();
    cache.set("temp".into(), "123".into(), 0);
    let removed = cache.remove(&"temp".to_string());
    assert_eq!(removed, Some("123".to_string()));
    assert_eq!(cache.get(&"temp".to_string()), None);
}

#[test]
fn test_overwrite_value() {
    let  cache = InMemoryKeyValueCache::<String, String>::new();
    cache.set("same".into(), "1".into(), 0);
    cache.set("same".into(), "99".into(), 0);
    assert_eq!(cache.get(&"same".to_string()), Some("99".to_string()));
}

#[test]
fn test_multiple_keys() {
    let  cache = InMemoryKeyValueCache::<String, String>::new();
    cache.set("k1".into(), "one".into(), 0);
    cache.set("k2".into(), "two".into(), 0);
    cache.set("k3".into(), "three".into(), 0);

    assert_eq!(cache.get(&"k1".to_string()), Some("one".to_string()));
    assert_eq!(cache.get(&"k2".to_string()), Some("two".to_string()));
    assert_eq!(cache.get(&"k3".to_string()), Some("three".to_string()));
}


// #[tokio::test]
// async fn test_ttl_expiration() {
//     let mut cache = InMemoryKeyValueCache::<String, String>::new();
//     let ttl_secs = 1;
//     cache.set("k1".into(), "v1".into(), ttl_secs);
//     assert_eq!(cache.get(&"k1".to_string()), Some("v1".to_string()));
//     tokio::time::sleep(Duration::from_secs(3)).await;
//     assert_eq!(cache.get(&"k1".to_string()),  None);
// }

#[tokio::test]
async fn test_ttl_valid_access() {
    let cache = InMemoryKeyValueCache::<String, String>::new();
    let ttl_secs = 20;
    cache.set("k1".into(), "v1".into(), ttl_secs);
    assert_eq!(cache.get(&"k1".to_string()), Some("v1".to_string()));
    tokio::time::sleep(Duration::from_secs(3)).await;
    assert_eq!(cache.get(&"k1".to_string()), Some("v1".to_string()));
}




