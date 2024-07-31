use std::path::Path;
use glob::glob;
use lambda_runtime::{run, service_fn, tracing, Error, Context, LambdaEvent};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transcriber::{init_s3client, get_video, put_transcript};
use std::process::Command;
use aws_sdk_s3::Client;
use tokio::fs::create_dir_all;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ItemDetails {
    etag: String,
    key: String,
    last_modified: f64,
    size: u64,
    storage_class: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct S3Items {
    items: Vec<ItemDetails>,
}

#[derive(Serialize)]
struct Response {
    message: String,
}


async fn function_handler(event: LambdaEvent<S3Items>) -> Result<Response, Error> {
    // Init S3 client
    let s3client = init_s3client().await.unwrap();
    // Env Vars
    dotenv::dotenv().ok();
    let video_bucket = dotenv::var("VIDEO_BUCKET").expect("VIDEO_BUCKET not set");
    let tscript_bucket = dotenv::var("TRANSCRIPT_BUCKET").expect("TRANSCRIPT_BUCKET not set");
    // Process event payload
    let items = event.payload.items;
    // Download videos to /tmp/videos/
    for item in items {
        tracing::info!("Processing: {}", item.key);
        match get_video(&s3client, &video_bucket, &item.key).await {
            Ok(_) => {
                tracing::info!("SUCCESS: Downloaded {}", item.key);
            },
            Err(e) => {
                tracing::error!("ERROR: Failed to download {}: {}", item.key, e);
            }
        }
    }
    // List the videos in /tmp/videos/
    let glob_pattern = "/tmp/videos/*.mp4".to_string();
    for entry in glob(&glob_pattern).expect("ERROR: Failed to glob *.mp4 files") {
        match entry {
            Ok(video_path) => {
                tracing::info!("Found video: {}", video_path.display());
            },
            Err(e) => {
                println!("Failed to read glob entry. {}", e)
            }
        }
    }
    // Transcribe videos in /tmp/videos/ --> /tmp/transcripts/
    tracing::info!("Transcribing videos");
    let output = Command::new("sh")
        .arg("-c")
        .arg("./transcribe.sh /tmp/videos")
        .spawn()
        .expect("ERROR: Failed to execute transcription command")
        .wait()
        .expect("ERROR: Failed to wait for transcription command");

    // let stdout = String::from_utf8(output.stdout).expect("ERROR: Failed to convert stdout to String");
    // let stderr = String::from_utf8(output.stderr).expect("ERROR: Failed to convert stderr to String");

    // Get /tmp/transcripts/ paths
    let glob_pattern = "/tmp/transcripts/**/video*.txt".to_string();
    for entry in glob(&glob_pattern).expect("ERROR: Failed to glob *.txt files") {
        match entry {
            Ok(tscript_path) => {
                // Upload to S3
                match put_transcript(&s3client, &tscript_bucket, &tscript_path).await {
                    Ok(resp) => {
                        tracing::info!("{}", resp);
                    },
                    Err(e) => {
                        tracing::error!("ERROR: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("Failed to read glob entry. {}", e)
            }
        }
    }

    // Prepare the response
    let resp = Response {
        message: "SUCCESS: Files processed".to_string()
    };

    // Return `Response` (it will be serialized to JSON automatically by the runtime)
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt().json()
        .with_max_level(tracing::Level::INFO)
        .with_current_span(false)
        .without_time()
        .with_target(false)
        .init();

    run(service_fn(function_handler)).await
}