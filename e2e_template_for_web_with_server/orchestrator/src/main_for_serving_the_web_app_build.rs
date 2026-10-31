//! Serves `web_app/dist/` over HTTP. Missing paths without a file
//! extension fall back to `index.html`, so client-side routes work.

use std::path::PathBuf;
use tokio::net::TcpListener;
use tower_http::services::{ServeDir, ServeFile};

pub async fn main_for_serving_the_web_app_build(
    listener: TcpListener,
) -> Result<(), String> {
    let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or("orchestrator crate has no parent folder")?
        .to_path_buf();
    let dist = workspace.join("web_app/dist");
    let index = dist.join("index.html");

    let app = axum::Router::new()
        .fallback_service(ServeDir::new(&dist).fallback(ServeFile::new(&index)));

    axum::serve(listener, app)
        .await
        .map_err(|e| format!("static server failed: {e}"))
}
