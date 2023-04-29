use std::num::NonZeroUsize;

use super::cache_item::CacheItem;
use super::config::CacheConfig;
use aws_sdk_config::error::SdkError;
use aws_sdk_secretsmanager::operation::get_secret_value::GetSecretValueError;
use aws_sdk_secretsmanager::{Client as SecretsManagerClient};
use lru::LruCache;

/// Client for in-process caching of secret values from AWS Secrets Manager.
///
/// An LRU (least-recently used) caching scheme is used that provides
/// O(1) insertions and O(1) lookups for cached values.
pub struct SecretCache {
    client: SecretsManagerClient,
    config: CacheConfig,
    cache: LruCache<String, CacheItem<String>>,
}

impl SecretCache {
    /// Returns a new SecretsCache using the default Cache Configuration options.
    pub fn new(client: SecretsManagerClient) -> Self {
        SecretCache::new_cache(client, CacheConfig::new())
    }

    /// Returns a new SecretsCache using a provided custom Cache Configuration.
    pub fn new_with_config(client: SecretsManagerClient, config: CacheConfig) -> Self {
        SecretCache::new_cache(client, config)
    }

    fn new_cache(client: SecretsManagerClient, config: CacheConfig) -> Self {
        let cache = LruCache::new(
            NonZeroUsize::new(config.max_cache_size)
                .unwrap_or(NonZeroUsize::new(1).expect("Default max_cache_size must be non-zero")),
        );
        Self {
            client,
            config,
            cache,
        }
    }

    /// Returns a builder for getting secret strings.
    ///
    /// Retrieve the secret value with send()
    pub fn get_secret_string(&mut self, secret_id: String) -> GetSecretStringBuilder {
        GetSecretStringBuilder::new(self, secret_id)
    }
}

/// A builder for the get_secret_string method.
pub struct GetSecretStringBuilder<'a> {
    secret_cache: &'a mut SecretCache,
    secret_id: String,
    force_refresh: bool,
}

impl<'a> GetSecretStringBuilder<'a> {
    pub fn new(secret_cache: &'a mut SecretCache, secret_id: String) -> Self {
        GetSecretStringBuilder {
            secret_cache,
            secret_id,
            force_refresh: false,
        }
    }

    /// Forces a refresh of the secret.
    ///
    /// Forces the secret to be fetched from AWS and updates the cache with the fresh value.
    /// This is required when the cached secret is out of date but not expired, for example due to rotation.
    pub fn force_refresh(mut self) -> Self {
        self.force_refresh = true;
        self
    }

    /// Fetches the secret value from the cache.
    ///
    /// If the secret value exists in the cache and hasn't expired it will be immediately returned.
    /// The secret will be fetched by calling AWS Secrets Manager and updated in the cache if:
    /// - the secret value hasn't been stored in the cache
    /// - the secret stored in the cache but has expired
    /// - the force_refresh option was provided
    ///
    /// Values are stored in the cache with the cache_item_ttl from the CacheConfig.
    pub async fn send(&mut self) -> Result<String, SdkError<GetSecretValueError>> {
        if !self.force_refresh {
            if let Some(cache_item) = self.secret_cache.cache.get(&self.secret_id) {
                if !cache_item.is_expired() {
                    return Ok(cache_item.value.clone());
                }
            }
        }

        match self.fetch_secret().await {
            Ok(secret_value) => {
                let cache_item = CacheItem::new(
                    secret_value.clone(),
                    self.secret_cache.config.cache_item_ttl,
                );
                self.secret_cache
                    .cache
                    .put(self.secret_id.clone(), cache_item);
                Ok(secret_value)
            }
            Err(e) => Err(e),
        }
    }

    async fn fetch_secret(&mut self) -> Result<String, SdkError<GetSecretValueError>> {
        match self
            .secret_cache
            .client
            .get_secret_value()
            .secret_id(self.secret_id.clone())
            .version_stage(self.secret_cache.config.version_stage.clone())
            .send()
            .await
        {
            Ok(resp) => return Ok(resp.secret_string.as_deref().unwrap().to_string()),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aws_sdk_secretsmanager::{Client as SecretsManagerClient, Config};
    use aws_sdk_config::config::{Credentials, Region};

    #[test]
    fn get_secret_string_builder_defaults() {
        let mock_secrets_manager_client = get_mock_secretsmanager_client();
        let mut secret_cache = SecretCache::new(mock_secrets_manager_client);

        let builder = GetSecretStringBuilder::new(&mut secret_cache, "service/secret".to_string());

        assert_eq!(builder.secret_id, "service/secret");
        assert!(!builder.force_refresh);
    }

    #[test]
    fn get_secret_string_builder_force_refresh() {
        let mock_secrets_manager_client = get_mock_secretsmanager_client();
        let mut secret_cache = SecretCache::new(mock_secrets_manager_client);

        let builder = GetSecretStringBuilder::new(&mut secret_cache, "service/secret".to_string())
            .force_refresh();

        assert_eq!(builder.secret_id, "service/secret");
        assert!(builder.force_refresh);
    }

    // provides a mocked AWS SecretsManager client for testing
    fn get_mock_secretsmanager_client() -> SecretsManagerClient {
        let conf = Config::builder()
            .region(Region::new("ap-southeast-2"))
            .credentials_provider(Credentials::new("asdf", "asdf", None, None, "test"))
            .build();

        SecretsManagerClient::from_conf(conf)
    }
}
