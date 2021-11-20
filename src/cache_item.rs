use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Copy)]
/// Stores a cached item value with an expiry TTL.
pub struct CacheItem<T> {
    /// The item value stored in the cache.
    pub value: T,

    /// The expiry time of the cached item.
    ///
    /// Defined as the number of nanoseconds elapsed since the unix epoch.
    ttl: u128,
}

impl<T> CacheItem<T> {
    /// Returns a cached item.
    ///
    /// Sets the TTL of the item to the current time in nanoseconds since the unix epoch
    /// plus the provided cache_item_ttl value.
    pub fn new(value: T, cache_item_ttl: u128) -> Self {
        CacheItem {
            value,
            ttl: current_time_in_nanoseconds() + cache_item_ttl,
        }
    }

    /// Determines whether the cached item has expired.
    ///
    /// Expiration is determined by comparing the current time
    /// in nanoseconds to the cached item's TTL value.
    pub fn is_expired(&self) -> bool {
        current_time_in_nanoseconds() > self.ttl
    }
}

// Helper function that returns the current nanoseconds since the UNIX epoch
fn current_time_in_nanoseconds() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{thread, time};

    #[test]
    fn cache_item_fresh() {
        // 1 hr in nanoseconds
        let cache_item_ttl: u128 = 3600000000000;
        let cache_item = CacheItem::new("secret_value", cache_item_ttl);

        assert_eq!(cache_item.value, "secret_value");
        assert_eq!(cache_item.is_expired(), false);
    }

    #[test]
    fn cache_item_expired() {
        let cache_item = CacheItem::new("secret_value", 0);

        // sleep to simulate value expiring
        let one_hundred_millis = time::Duration::from_millis(100);
        thread::sleep(one_hundred_millis);

        assert_eq!(cache_item.value, "secret_value");
        assert_eq!(cache_item.is_expired(), true);
    }
}
