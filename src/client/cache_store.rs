use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Debug)]
struct CacheValue {
    value: String,
    expires_at: Option<Instant>,
}

#[derive(Debug)]
pub struct CacheStore {
    data: HashMap<String, CacheValue>,
}

impl CacheStore {
    pub fn new() -> Self {
        CacheStore {
            data: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String, expiry: Option<Duration>) {
        let expires_at = expiry.map(|expiry| Instant::now() + expiry);
        self.data.insert(key, CacheValue { value, expires_at });
    }
    pub fn get(&self, key: &str) -> Option<String> {
        self.data.get(key).and_then(|cache_value| {
            if cache_value
                .expires_at
                .map_or(true, |expiry| expiry > Instant::now())
            {
                Some(cache_value.value.clone())
            } else {
                None
            }
        })
    }
}
