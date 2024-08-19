use serde::{Deserialize, Serialize};
use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};
use cleanup::{init_s3client, delete_video};

#[derive(Deserialize)]
struct TranscriberDetails {
    message: String,
    processed: Vec<String>,
    failed: Vec<String>,
}

#[derive(Serialize)]
struct CleanupResponse {
    message: String,
    processed: Vec<String>,
    failed: Vec<String>,
}


#[tracing::instrument(skip(event), fields(req_id = %event.context.request_id))]
async fn function_handler(event: LambdaEvent<Vec<TranscriberDetails>>) -> Result<CleanupResponse, Error> {
    dotenv::dotenv().ok();
    let video_bucket = dotenv::var("VIDEO_BUCKET").expect("VIDEO_BUCKET not set");
    // Process event payload
    let items = event.payload;
    // Init s3 client
    let s3client = init_s3client().await.unwrap();
    // Cleanup
    let mut processed_videos: Vec<String> = vec![];
    let mut failed_videos: Vec<String> = vec![];
    for item in items {
        // for processed transcripts only
        for transcript in item.processed {
            // change extension from .txt to .mp4
            let key = transcript.replace(".txt", ".mp4");
            // delete videos
            match delete_video(&s3client, &video_bucket, &key).await {
                Ok(resp) => {
                    match resp.status {
                        200 => processed_videos.push(resp.key),
                        400 => failed_videos.push(resp.key),
                        _ => tracing::info!("ERROR: Unknown status for DeleteResponse")
                    }
                    tracing::info!("{}", resp.message);
                },
                Err(e) => {
                    tracing::error!("ERROR: {}", e);
                }
            }
        }
    }
    // Prepare the response
    let resp = CleanupResponse {
        message: format!("CLEANUP COMPLETE: {}", video_bucket),
        processed: processed_videos,
        failed: failed_videos
    };

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
