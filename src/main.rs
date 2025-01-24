use std::time::Duration;

use reqwest::ClientBuilder;
use tokio::time::sleep;
// use tokio::select;
use tracing::{error, info};
use tracing_subscriber::fmt;

#[tokio::main]
async fn main() {
    fmt::init();
    let client = ClientBuilder::new()
        .http2_prior_knowledge()
        .connection_verbose(true)
        .danger_accept_invalid_hostnames(false)
        .build()
        .unwrap();

    loop {
        // select! {
        //     result = client.get("https://127.0.0.1:27000/app_status").send() => match result {
        //         Ok(response) => info!(status = %response.status(), "Response status"),
        //         Err(err) => error!(?err, "Got error"),
        //     },
        //     _ = sleep(Duration::from_secs(1)) => info!("Timed out")
        // }
        match client
            .get("https://sandbox.imedidata.net/app_status")
            // .get("https://127.0.0.1:27000/app_status")
            .send()
            .await
        {
            Ok(response) => info!(status = %response.status(), "Response status"),
            Err(err) => error!(?err, "Got error"),
        }
        sleep(Duration::from_secs(1)).await;
    }
}
