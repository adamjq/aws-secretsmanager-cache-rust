// Apache License

// Copyright (c) 2021 Adam Quigley

//! This crate provides a client for in-process caching of secrets from AWS Secrets Manager for Rust applications.
//! It is heavily inspired by the [AWS Secrets Manager Go Caching Client](https://github.com/aws/aws-secretsmanager-caching-go)
//! and the [AWS SDK for Rust](https://github.com/awslabs/aws-sdk-rust).
//!
//! The client internally uses an LRU (least-recently used) caching scheme that provides
//! O(1) insertions and O(1)lookups for cached values.

//! ## Example
//! ```rust
//! use aws_sdk_secretsmanager::Client;
//! use aws_secretsmanager_cache::SecretCache;
//!
//! #[tokio::main]
//! async fn main() {
//!     let aws_config = aws_config::from_env().load().await;
//!     let client = Client::new(&aws_config);
//!     let mut cache = SecretCache::new(client);
//!
//!     let secret_id = "service/secret";
//!
//!     match cache.get_secret_string(secret_id.to_string()).send().await {
//!         Ok(secret_value) => {
//!             // do something
//!         }
//!         Err(e) => println!("{}", e),
//!     }
//! }
//! ```

mod cache;
mod cache_item;
mod config;
pub use cache::SecretCache;
pub use config::CacheConfig;
