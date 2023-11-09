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
    let secret_id = "service/secret";

    match CACHE
        .get() // get cache from the global scope
        .await
        .lock() // acquire cache lock
        .unwrap()
        .get_secret_string(secret_id)
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
