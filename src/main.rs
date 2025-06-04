use anyhow::Result;
use aws_config::BehaviorVersion;
use aws_sdk_s3::Client;
use recallmon::{api::AppState, run_server, wal::WalAppender};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let config = aws_config::defaults(BehaviorVersion::latest()).load().await;
    let client = Client::new(&config);
    let wal = WalAppender {
        bucket: std::env::var("RECALLMON_BUCKET").unwrap_or_else(|_| "recallmon".into()),
        client,
    };

    let state = AppState { wal: Arc::new(wal) };
    let addr = "0.0.0.0:3000".parse()?;
    run_server(addr, state).await?;
    Ok(())
}
