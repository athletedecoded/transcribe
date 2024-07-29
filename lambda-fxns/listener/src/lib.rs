use aws_sdk_sfn::{Client, Error};
use aws_config::BehaviorVersion;

// Initialize step function client
pub async fn init_client() -> Result<Client, Error> {
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let client = Client::new(&config);
    Ok(client)
}