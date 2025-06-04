use crate::models::VectorRecord;
use crate::wal::WalAppender;
use anyhow::Result;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::{routing::post, Router};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub wal: Arc<WalAppender>,
}

pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/namespace/:ns/doc", post(put_document))
        .with_state(state)
}

pub async fn run_server(addr: std::net::SocketAddr, state: AppState) -> Result<()> {
    let app = app(state);
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

async fn put_document(
    State(state): State<AppState>,
    axum::Json(rec): axum::Json<VectorRecord>,
) -> impl IntoResponse {
    match state.wal.append(&rec).await {
        Ok(_) => axum::http::StatusCode::OK,
        Err(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
    }
}
