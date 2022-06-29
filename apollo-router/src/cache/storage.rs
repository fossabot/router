use lru::LruCache;
use std::hash::Hash;

use std::sync::Arc;
use tokio::sync::Mutex;
// placeholder storage module
//
// this will be replaced by the multi level (in memory + redis/memcached) once we find
// a suitable implementation.
#[derive(Clone)]
pub(crate) struct CacheStorage<K: Hash + Eq + Send, V: Clone> {
    inner: Arc<Mutex<LruCache<K, V>>>,
}

impl<K, V> CacheStorage<K, V>
where
    K: Hash + Eq + Send,
    V: Clone + Send,
{
    pub(crate) async fn new(max_capacity: usize) -> Self {
        Self {
            inner: Arc::new(Mutex::new(LruCache::new(max_capacity))),
        }
    }

    pub(crate) async fn get(&self, key: &K) -> Option<V> {
        self.inner.lock().await.get(key).cloned()
    }

    pub(crate) async fn insert(&mut self, key: K, value: V) {
        self.inner.lock().await.put(key, value);
    }
}