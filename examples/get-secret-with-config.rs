use aws_sdk_secretsmanager::Client;
use aws_secretsmanager_cache::{CacheConfig, SecretCache};
use std::time;

#[tokio::main]
async fn main() {
    let aws_config = aws_config::from_env().load().await;
    let client = Client::new(&aws_config);

    let custom_cache_ttl = time::Duration::from_secs(30).as_nanos();
    let cache_config = CacheConfig::new().cache_item_ttl(custom_cache_ttl);

    let mut cache = SecretCache::new_with_config(client, cache_config);

    let secret_id = "service/secret";

    match cache
        .get_secret_string(secret_id.to_string())
        .force_refresh() // force the value to be fetched from AWS and updated in the cache
        .send()
        .await
    {
        Ok(secret_value) => {
            println!(
                "Successfully retrieved secret {}: {}",
                secret_id, secret_value
            );
        }
        // e.g. ResourceNotFoundException: Secrets Manager can't find the specified secret.
        Err(e) => println!("ERROR: Error getting secret '{}'. {}", secret_id, e),
    }
}
