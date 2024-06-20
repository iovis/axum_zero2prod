use axum::http::StatusCode;
use axum::{routing::get, Router};
use tokio::net::TcpListener;

pub async fn run(listener: TcpListener) -> Result<(), std::io::Error> {
    tracing_subscriber::fmt::init();
    let app = Router::new().route("/health_check", get(health_check));
    axum::serve(listener, app).await
}

#[axum::debug_handler]
pub async fn health_check() -> StatusCode {
    StatusCode::OK
}
