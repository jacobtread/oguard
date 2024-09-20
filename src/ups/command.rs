use std::{
    any::Any,
    collections::HashMap,
    time::{Duration, Instant},
};

use compact_str::CompactString;

/// Trait implemented by structures that can be used
/// as device commands
pub trait IntoDeviceCommand: Send + 'static {
    /// Response type to decode
    type Response: FromDeviceResponse;

    /// Gets the command string to send to the device
    fn get_command(&self) -> CompactString;

    /// Unique cache key to use if the response can be cached
    fn cache_key(&self) -> Option<u64> {
        None
    }

    /// Invalidates any cache keys this command will have an affect on.
    /// Used by queries that manipulate the device in order to discard
    /// cached device states
    fn invalidate_cache(&self, _cache: &mut ResponseCache) {}
}

pub trait FromDeviceResponse: Send + Clone + 'static {
    fn from_device_response(value: CompactString) -> anyhow::Result<Self>;
}

impl FromDeviceResponse for () {
    #[inline]
    fn from_device_response(_value: CompactString) -> anyhow::Result<Self> {
        Ok(())
    }
}

/// Cache for storing device command responses
#[derive(Default)]
pub struct ResponseCache {
    cache: HashMap<u64, CachedValue>,
}

/// Cached value inside of [ResponseCache] for caching
/// a response with an expiry time
struct CachedValue {
    /// Actual cached response value
    value: Box<dyn Any + Send>,
    /// Instant the entry becomes invalid
    expires_at: Instant,
}

impl ResponseCache {
    /// Time responses should be cached for before expiring
    const CACHE_TIME: Duration = Duration::from_secs(1);

    /// Attempts to get an item from the response cache using its
    /// cache key
    pub fn get<T>(&mut self, cache_key: Option<u64>) -> Option<T>
    where
        T: Send + Clone + 'static,
    {
        let cache_key = cache_key?;
        let cache_value = self.cache.get(&cache_key)?;

        if cache_value.expires_at <= Instant::now() {
            self.remove(cache_key);
            return None;
        }

        let value = cache_value.value.downcast_ref::<T>();
        value.cloned()
    }

    /// Inserts a response into the cache using its cache key
    pub fn insert<T>(&mut self, cache_key: u64, value: &T)
    where
        T: Send + Clone + 'static,
    {
        let value = value.clone();
        let cache_value = CachedValue {
            expires_at: Instant::now() + Self::CACHE_TIME,
            value: Box::new(value),
        };

        self.cache.insert(cache_key, cache_value);
    }

    /// Removes any cached values for the provided cache key
    #[inline]
    pub fn remove(&mut self, cache_key: u64) {
        self.cache.remove(&cache_key);
    }
}
