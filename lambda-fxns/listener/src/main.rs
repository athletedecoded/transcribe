use serde::Serialize;
use aws_lambda_events::event::s3::S3Event;
use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};
use listener::init_client;

#[derive(Serialize)]
struct Response {
    message: String,
}

#[tracing::instrument(skip(event), fields(req_id = %event.context.request_id))]
async fn function_handler(event: LambdaEvent<S3Event>) -> Result<Response, Error> {
    dotenv::dotenv().ok();
    let state_machine = dotenv::var("STATE_MACHINE_ARN").expect("STATE_MACHINE_ARN not set");
    // Validate trigger config
    let event_type = event.payload.records[0]
        .event_name
        .as_ref()
        .unwrap()
        .as_str();
    let bucket = event.payload.records[0].s3.bucket.name.as_ref().unwrap().as_str();
    tracing::info!("{event_type} event trigger on bucket {}",event_type);
    // Listen for done file
    let key = event.payload.records[0].s3.object.key.as_ref().unwrap().as_str();
    let response = match key {
        "done.txt" => {
            // Initialize client
            let sfn_client = init_client().await.unwrap();
            // Start execution
            let payload = r#"{"input": {"payload": "Listener --> Step Function!"}}"#;
            let resp = sfn_client.start_execution()
                .state_machine_arn(&state_machine)
                .input(payload)
                .send()
                .await
                .unwrap();

            println!("Step function response: `{:?}`", resp);

            // Trigger step function
            "Step function triggered".to_string()
        }
        _ => {
            format!("UPLOAD: {}", &key)
        }
    };
    tracing::info!(response);
    Ok(Response {message: response})
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
