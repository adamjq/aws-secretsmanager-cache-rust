const DEFAULT_MAX_CACHE_SIZE: usize = 1024;
const DEFAULT_CACHE_ITEM_TTL: u128 = 3600000000000; // 1 hour in nanoseconds
const DEFAULT_VERSION_STAGE: &str = "AWSCURRENT";

/// Configuration options for the SecretCache.
///
/// Defaults:
/// - max_cache_size: 1024
/// - cache_item_ttl: 3600000000000 (1hr)
/// - version_stage: "AWSCURRENT"
pub struct CacheConfig {
    /// The maximum number of secrets to maintain in the cache.
    ///
    /// The least frequently accessed items will be evicted from the cache
    /// once a max_cache_size number of items are stored.
    ///
    /// Default: 1024
    pub max_cache_size: usize,

    /// The TTL expiry of items in the cache.
    ///
    /// Determines the number of nanoseconds a cached secret will be considered valid before
    /// the secret value is required to be refreshed. Refreshing happens synchronously.
    ///
    /// Default: 3600000000000 (1 hour in nanoseconds)
    pub cache_item_ttl: u128,

    /// The version stage used when requesting secrets from AWS Secrets Manager.
    ///
    /// Default: "AWSCURRENT"
    pub version_stage: String,
}

impl CacheConfig {
    /// Returns a new Cache Configuration with default values set.
    ///
    /// Defaults:
    /// - max_cache_size: 1024
    /// - cache_item_ttl: 3600000000000 (1hr)
    /// - version_stage: "AWSCURRENT"
    pub fn new() -> Self {
        CacheConfig {
            max_cache_size: DEFAULT_MAX_CACHE_SIZE,
            cache_item_ttl: DEFAULT_CACHE_ITEM_TTL,
            version_stage: DEFAULT_VERSION_STAGE.to_string(),
        }
    }

    /// Sets the max_cache_size cache configuration option to a different value.
    pub fn max_cache_size(mut self, max_cache_size: usize) -> Self {
        self.max_cache_size = max_cache_size;
        self
    }

    /// Sets the cache_item_ttl cache configuration option to a different value.
    pub fn cache_item_ttl(mut self, cache_item_ttl: u128) -> Self {
        self.cache_item_ttl = cache_item_ttl;
        self
    }
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time;

    #[test]
    fn cache_config_default() {
        let cache_config = CacheConfig::new();

        assert_eq!(cache_config.cache_item_ttl, DEFAULT_CACHE_ITEM_TTL);
        assert_eq!(cache_config.max_cache_size, DEFAULT_MAX_CACHE_SIZE);
        assert_eq!(cache_config.version_stage, DEFAULT_VERSION_STAGE);
    }

    #[test]
    fn cache_config_custom() {
        let custom_cache_ttl = time::Duration::from_secs(30).as_nanos();
        let cache_config = CacheConfig::new()
            .max_cache_size(10)
            .cache_item_ttl(custom_cache_ttl);

        assert_eq!(cache_config.cache_item_ttl, custom_cache_ttl);
        assert_eq!(cache_config.max_cache_size, 10);
        assert_eq!(cache_config.version_stage, DEFAULT_VERSION_STAGE);
    }

    #[test]
    fn cache_config_partial_config() {
        let cache_config = CacheConfig::new().max_cache_size(10);

        assert_eq!(cache_config.cache_item_ttl, DEFAULT_CACHE_ITEM_TTL);
        assert_eq!(cache_config.max_cache_size, 10);
        assert_eq!(cache_config.version_stage, DEFAULT_VERSION_STAGE);
    }
}
