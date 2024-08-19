use aws_sdk_s3::{Client, Error};
use aws_config::BehaviorVersion;

pub struct DeleteResponse {
    pub key: String,
    pub status: i32,
    pub message: String
}

// Init S3 Client
pub async fn init_s3client() -> Result<Client, Error> {
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let client = Client::new(&config);
    Ok(client)
}

pub async fn delete_video(client: &Client, bucket: &str, key: &str) -> Result<DeleteResponse, Error> {
    match client.delete_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await {
        Ok(_) => {
            Ok(DeleteResponse {
                key: key.to_string(),
                status: 200,
                message: format!("SUCCESS: deleted {}", key)
            })
        }
        Err(e) => {
            Ok(DeleteResponse {
                key: key.to_string(),
                status: 400,
                message: format!("ERROR: Failed to delete {} : {}", key, e)
            })
        }
    }
}