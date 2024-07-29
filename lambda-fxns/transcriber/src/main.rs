use lambda_runtime::{run, service_fn, tracing, Error, Context, LambdaEvent};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use transcriber::{init_s3client, get_video};

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
    // Process event payload
    let items = event.payload.items;
    // Download videos to /tmp/videos/
    for item in items {
        let bucket_name = "whisper-videos";
        tracing::info!("Processing: {}", item.key);
        match get_video(&s3client, &bucket_name, &item.key).await {
            Ok(_) => {
                tracing::info!("SUCCESS: Downloaded {}", item.key);
            },
            Err(e) => {
                tracing::error!("ERROR: Failed to download {}: {}", item.key, e);
            }
        }
    }
    // Prepare the response
    let resp = Response {
        message: "Videos Downloaded".to_string(),
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