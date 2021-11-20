# AWS Secrets Manager Rust Caching Client

This crate provides a client for in-process caching of secrets from AWS Secrets Manager for Rust applications. 
It's heavily inspired by the [AWS Secrets Manager Go Caching Client](https://github.com/aws/aws-secretsmanager-caching-go) 
and the [AWS SDK for Rust](https://github.com/awslabs/aws-sdk-rust).

The client internally uses an LRU (least-recently used) caching scheme that provides 
O(1) insertions and O(1) lookups for cached values.

**Please Note: This client depends on the AWS SDK for Rust which is currently in an alpha release state. The functionality of this client should therefore be considered an alpha release and may change in the future**

## Getting started

To use this client you must have:
- A Rust development environment
- An Amazon Web Services (AWS) account to access secrets stored in AWS Secrets Manager and use AWS SDK for Rust.

## Usage

The following sample demonstrates how to get started using the client:

```rust
use aws_sdk_secretsmanager::Client;
use aws_secretsmanager_cache::SecretCache;

#[tokio::main]
async fn main() {
    // instantiate an AWS SecretsManager client using the AWS Rust SDK
    let aws_config = aws_config::from_env().load().await;
    let client = Client::new(&aws_config);
    
    let mut cache = SecretCache::new(client);

    match cache.get_secret_string("YOUR_SECRET_ID".to_string()).send().await {
        Ok(secret_value) => {
            // use secret value
        }
        // e.g. ResourceNotFoundException: Secrets Manager can't find the specified secret.
        Err(e) => println!("ERROR: {}", e),
    }
}
```

### Forcing cache refreshes

If a secret has been rotated since the last value was fetched and cached, and hasn't expired in the cache, it's necessary to force a cache refresh for the value by calling AWS and updating the value.

This can be done with `force_refresh()`, for example:

```rust
    match cache
        .get_secret_string("YOUR_SECRET_ID".to_string())
        .force_refresh()
        .send()
        .await
```

## Cache Configuration

- `max_cache_size usize` The maximum number of secrets to maintain in the cache 
before evicting the least frequently accessed
- `cache_item_ttl u128` The number of nanoseconds a cached secret will be considered 
valid before the secret value requires a refresh. Refreshing happens synchronously.

```rust
use aws_sdk_secretsmanager::Client;
use aws_secretsmanager_cache::{CacheConfig, SecretCache};
use std::time;

#[tokio::main]
async fn main() {
    let aws_config = aws_config::from_env().load().await;
    let client = Client::new(&aws_config);

    // cache configuration with 30 second expiry time and maximum 1000 secrets
    let cache_config = CacheConfig::new()
        .cache_item_ttl(time::Duration::from_secs(30).as_nanos())
        .max_cache_size(1000);

    let mut cache = SecretCache::new_with_config(client, cache_config);
}
```

## Global Caching

Certain cloud environments like AWS Lambda encourage initializing clients in the global scope to avoid initialization for
each function invocation. This can be achieved using the `lazy_static` crate, for example: 

```rust
use async_once::AsyncOnce;
use aws_sdk_secretsmanager::Client;
use aws_secretsmanager_cache::SecretCache;
use lazy_static::lazy_static;
use std::sync::Mutex;

// store the cache in the global scope - useful for runtime environments like AWS Lambda
lazy_static! {
    static ref CACHE: AsyncOnce<Mutex<SecretCache>> = AsyncOnce::new(async {
        Mutex::new(SecretCache::new(Client::new(
            &aws_config::from_env().load().await,
        )))
    });
}

#[tokio::main]
async fn main() {
    // use cache
}
```

## License

Licensed under the [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0) or the [MIT license](https://opensource.org/licenses/MIT), at your option. Files in the project may not be copied, modified, or distributed except according to those terms.
