use aws_sdk_secretsmanager::Client;
use aws_secretsmanager_cache::SecretCache;

#[tokio::main]
async fn main() {
    let aws_config = aws_config::from_env().load().await;
    let client = Client::new(&aws_config);
    let mut cache = SecretCache::new(client);

    let secret_id = "service/secret";

    match cache.get_secret_string(secret_id).send().await {
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
