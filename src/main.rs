use clap::Parser;
use glob::glob;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::process;
use transcribe::{extract_key, init_s3client, upload_object, validate_config, validate_path};

#[derive(Parser, Default, Debug)]
#[clap(
    version = "1.0",
    author = "Kahlia Hogg",
    about = "Transcriber",
    after_help = "Example: ./transcribe /path/to/vid_dir"
)]
struct Args {
    vid_dir: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // load config
    dotenv::dotenv().ok();
    let args = Args::parse();
    let vid_dir = Path::new(&args.vid_dir);
    let vid_bucket = dotenv::var("VIDEO_BUCKET").expect("ERROR: VIDEO_BUCKET not set");
    let tscript_bucket =
        dotenv::var("TRANSCRIPT_BUCKET").expect("ERROR: TRANSCRIPT_BUCKET not set");
    let s3client = init_s3client().await.unwrap();
    // Run config checks
    match validate_config(&s3client, vid_dir, &vid_bucket, &tscript_bucket).await {
        Ok(_) => println!("Config validated"),
        Err(e) => {
            println!("ERROR: {}", e);
            process::exit(1);
        }
    }
    // get all videos in vids_dir and subdirs
    let glob_pattern = format!("{}/**/video*.mp4", args.vid_dir);
    for entry in glob(&glob_pattern).expect("ERROR: Failed to glob *.mp4 files") {
        match entry {
            Ok(vid_path) => {
                // Check video path matches convention
                match validate_path(&vid_path.to_string_lossy()) {
                    Ok(_) => (),
                    Err(e) => {
                        println!("ERROR: {}", e);
                        break;
                    }
                }
                // Extract key from path
                let key = match extract_key(&vid_path).await {
                    Some(key) => key,
                    None => {
                        println!("ERROR: Failed to extract key from {}", vid_path.display());
                        break;
                    }
                };
                // Send to S3
                match upload_object(&s3client, &vid_bucket, &vid_path, &key).await {
                    Ok(_) => println!("SUCCESS: uploaded {}", vid_path.display()),
                    Err(e) => {
                        println!("ERROR: Failed to upload {}. {}", vid_path.display(), e);
                    }
                }
            }
            Err(e) => println!("Failed to read glob entry. {}", e),
        }
    }
    // create & upload done file
    let done_path = Path::new("done.txt");
    let _file = File::create(&done_path);
    match upload_object(&s3client, &vid_bucket, &done_path, "done.txt").await {
        Ok(_) => println!("SUCCESS: Upload complete for {}", vid_dir.display()),
        Err(e) => println!("ERROR: Failed to upload done file. {}", e),
    }

    Ok(())
}
