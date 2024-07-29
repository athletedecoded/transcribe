use aws_config::BehaviorVersion;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::{Client, Error};
use regex::Regex;
use std::path::{Path, PathBuf};
use std::process;

// Create S3 client
pub async fn init_s3client() -> Result<Client, Error> {
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let client = Client::new(&config);
    Ok(client)
}

// Check config
pub async fn validate_config(
    client: &Client,
    vid_dir: &Path,
    vid_bucket: &str,
    tscript_bucket: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // validate vid_dir
    if !vid_dir.is_dir() {
        return Err(format!("{} is not a valid directory", vid_dir.display()).into());
    }
    // validate video upload bucket
    if !bucket_exists(client, vid_bucket).await? {
        return Err(format!("{} does not exist", vid_bucket).into());
    }
    // validate transcript upload bucket
    if !bucket_exists(client, tscript_bucket).await? {
        return Err(format!("{} does not exist", tscript_bucket).into());
    }
    Ok(())
}

pub fn validate_path(vid_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Incoming paths are of the format "*/video*.mp4"
    // Check if the path matches the expected pattern */week##/lesson##/video##.mp4

    // Stage 0: Video must be numbered
    let pattern0 = r".*/video\d+\.mp4";
    let re0 = Regex::new(pattern0).unwrap();
    if !re0.is_match(vid_path) {
        return Err(format!(
            "Invalid path format {}. Video id must be strictly numbered i.e **/video##.mp4",
            vid_path
        )
        .into());
    }
    // Stage 1: Lesson subdir
    let pattern1 = r".*/lesson\d+/video\d+\.mp4";
    let re1 = Regex::new(pattern1).unwrap();
    if !re1.is_match(vid_path) {
        return Err(format!("Invalid path format {}. Videos must be strictly within 'lesson##' directory i.e. **/lesson##/video##.mp4", vid_path).into());
    }
    // Stage 2: Week subdir
    let pattern2 = r".*/week\d+/lesson\d+/video\d+\.mp4";
    let re2 = Regex::new(pattern2).unwrap();
    if !re2.is_match(vid_path) {
        return Err(format!("Invalid path format {}. Videos must be strictly within 'week##/lesson##' directory i.e. */week##/lesson##/video##.mp4", vid_path).into());
    }

    Ok(())
}

// Check bucket exists
pub async fn bucket_exists(client: &Client, bucket_name: &str) -> Result<bool, Error> {
    let resp = client.list_buckets().send().await?;
    let buckets = resp.buckets();
    for bucket in buckets {
        if bucket.name().unwrap_or_default() == bucket_name {
            return Ok(true);
        }
    }
    Ok(false)
}

// Extract the video key (week##/lesson##/video##.mp4) from full path (path/to/vid_dir/week##/lesson##/video##.mp4)
pub async fn extract_key(path: &Path) -> Option<String> {
    let split_pos = path
        .iter()
        .position(|x| x.to_string_lossy().starts_with("week"))?;
    let key_buf: PathBuf = path.iter().skip(split_pos).collect();
    let key = key_buf.to_string_lossy().into_owned();
    Some(key)
}

// Put object in bucket
pub async fn upload_object(
    client: &Client,
    bucket: &str,
    object_path: &Path,
    key: &str,
) -> Result<(), Error> {
    let body = ByteStream::from_path(object_path).await;
    match body {
        Ok(b) => {
            let _resp = client
                .put_object()
                .bucket(bucket)
                .key(key)
                .body(b)
                .send()
                .await?;
        }
        Err(e) => {
            println!("ERROR: Failed to create bytestream");
            println!("{e}");
            process::exit(1);
        }
    }

    Ok(())
}
