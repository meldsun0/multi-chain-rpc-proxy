use std::hash::Hash;
use std::sync::Arc;
use dashmap::DashMap;

pub trait KeyValueCache<K, V>: Send + Sync {
    fn get(&self, key: &K) -> Option<V>;
    fn set(&mut self, key: K, value: V);
    fn remove(&mut self, key: &K) -> Option<V>;
    fn contains(&self, key: &K) -> bool;
}

pub struct InMemoryKeyValueCache<K, V> {
    store: Arc<DashMap<K, V>>,
}

impl<K, V> InMemoryKeyValueCache<K, V>
where
    K: Eq + Hash,
{
    pub fn new() -> Self {
        Self {
            store: Arc::new(DashMap::new()),
        }
    }
}

impl<K, V> KeyValueCache<K, V> for InMemoryKeyValueCache<K, V>
where
    K: Eq + Hash + Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    fn get(&self, key: &K) -> Option<V> {
        self.store.get(key).map(|v| v.clone())
    }

    fn set(&mut self, key: K, value: V) {
        self.store.insert(key, value);
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        self.store.remove(key).map(|(_, v)| v)
    }

    fn contains(&self, key: &K) -> bool {
        self.store.contains_key(key)
    }
}