use recallmon::{run_server, api::AppState, wal::WalAppender};
use std::sync::Arc;
use aws_sdk_s3::Client;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let config = aws_config::load_from_env().await;
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
