use rpc_proxy_meldsun0::cache::cache::{InMemoryKeyValueCache, KeyValueCache};

#[test]
fn test_set_and_get() {
    let mut cache = InMemoryKeyValueCache::<String, String>::new();
    cache.set("a".into(), "10".into());
    assert_eq!(cache.get(&"a".to_string()), Some("10".to_string()));
}

#[test]
fn test_contains() {
    let mut cache = InMemoryKeyValueCache::<String, String>::new();
    cache.set("key".into(), "42".into());
    assert!(cache.contains(&"key".to_string()));
    assert!(!cache.contains(&"not_found".to_string()));
}

#[test]
fn test_remove() {
    let mut cache = InMemoryKeyValueCache::<String, String>::new();
    cache.set("temp".into(), "123".into());
    let removed = cache.remove(&"temp".to_string());
    assert_eq!(removed, Some("123".to_string()));
    assert_eq!(cache.get(&"temp".to_string()), None);
}

#[test]
fn test_overwrite_value() {
    let mut cache = InMemoryKeyValueCache::<String, String>::new();
    cache.set("same".into(), "1".into());
    cache.set("same".into(), "99".into());
    assert_eq!(cache.get(&"same".to_string()), Some("99".to_string()));
}

#[test]
fn test_multiple_keys() {
    let mut cache = InMemoryKeyValueCache::<String, String>::new();
    cache.set("k1".into(), "one".into());
    cache.set("k2".into(), "two".into());
    cache.set("k3".into(), "three".into());

    assert_eq!(cache.get(&"k1".to_string()), Some("one".to_string()));
    assert_eq!(cache.get(&"k2".to_string()), Some("two".to_string()));
    assert_eq!(cache.get(&"k3".to_string()), Some("three".to_string()));
}
