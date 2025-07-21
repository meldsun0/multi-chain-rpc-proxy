use std::hash::Hash;
use std::sync::Arc;
use std::time::{Duration};
use dashmap::DashMap;
use tokio::time::Instant;

pub trait KeyValueCache<K, V>: Send + Sync {
    fn get(&self, key: &K) -> Option<V>;
    fn set(&self, key: K, value: V, ttl: u64);
    fn remove(&self, key: &K) -> Option<V>;
    fn contains(&self, key: &K) -> bool;
}

pub struct InMemoryKeyValueCache<K, V> {
    store: Arc<DashMap<K, V>>,
    ttl: Arc<DashMap<K, Option<Instant>>>,
}

impl<K, V> InMemoryKeyValueCache<K, V>
where
    K: Eq + Hash,
{
    pub fn new() -> Self {
        Self {
            store: Arc::new(DashMap::new()),
            ttl:   Arc::new(DashMap::new()),
        }
    }
}

impl<K, V> KeyValueCache<K, V> for InMemoryKeyValueCache<K, V>
where
    K: Eq + Hash + Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    fn get(&self, key: &K) -> Option<V> {
        let expired = self.ttl.get(key)
            .and_then(|guard| guard.clone())
            .map_or(false, |exp| Instant::now() >= exp);

        if expired {
            self.ttl.remove(key);
            self.store.remove(key);
            return None;
        }
        self.store.get(key).map(|v| v.clone())
    }

    fn set(&self, key: K, value: V, ttl: u64) {
        let expires_at = if ttl == 0 {
            None
        } else {
            Some(Instant::now() + Duration::from_secs(ttl))
        };
        self.store.insert(key.clone(), value);
        self.ttl.insert(key, expires_at);
    }

    fn remove(&self, key: &K) -> Option<V> {
        self.ttl.remove(key);
        self.store.remove(key).map(|(_, v)| v)
    }

    fn contains(&self, key: &K) -> bool {
        self.store.contains_key(key)
    }
}