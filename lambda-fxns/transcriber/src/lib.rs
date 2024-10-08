use aws_sdk_s3::{Client, Error};
use aws_config::BehaviorVersion;
use aws_sdk_s3::primitives::ByteStream;
use tokio::fs::{File, create_dir_all};
use tokio::io::copy;
use std::path::Path;


pub struct PutResponse {
    pub key: String,
    pub status: i32,
    pub message: String
}

// Create S3 client
pub async fn init_s3client() -> Result<Client, Error> {
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let client = Client::new(&config);
    Ok(client)
}

pub async fn get_video(client: &Client, bucket: &str, key: &str) -> Result<(), Error> {
    // Get response
    let resp = client.get_object().bucket(bucket).key(key).send().await?;
    // Get video as byte stream from response body
    let mut stream = resp.body.into_async_read();
    // Create a file to write the video data to
    let tmp_path = format!("/tmp/videos/{}", key);
    let dir_path = Path::new(&tmp_path).parent().expect("No parent directory found");
    create_dir_all(&dir_path).await.expect("Failed to create directories");
    let mut tmp_file = File::create(&tmp_path).await.unwrap();
    // Write the video data into the file
    let _file_msg = copy(&mut stream, &mut tmp_file).await.unwrap();
    Ok(())
}

pub async fn put_transcript(client: &Client, bucket: &str, filepath: &Path) -> Result<PutResponse, Error> {
    let stream = ByteStream::from_path(filepath).await;
    let path_str = filepath.display().to_string();
    let key = path_str.strip_prefix("/tmp/transcripts/").unwrap();
    match stream {
        Ok(body) => {
            match client.put_object()
                .bucket(bucket)
                .key(key)
                .body(body)
                .send()
                .await {
                    Ok(_) => {
                        Ok(PutResponse {
                            key: key.to_string(),
                            status: 200,
                            message: format!("SUCCESS: S3 upload {}", key)
                        })
                    }
                    Err(e) => {
                        Ok(PutResponse {
                            key: key.to_string(),
                            status: 400,
                            message: format!("ERROR: Failed S3 upload {} : {}", key, e)
                        })
                }
            }
        }
        Err(e) => {
            Ok(PutResponse {
                key: key.to_string(),
                status: 400,
                message: format!("ERROR: Failed to extract bytestream from {}, {}", key, e)
            })
        }
    }
}