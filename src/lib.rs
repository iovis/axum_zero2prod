use axum::http::StatusCode;
use axum::{routing::get, Router};

pub async fn run() -> Result<(), std::io::Error> {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/health_check", get(health_check));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await
}

#[axum::debug_handler]
pub async fn health_check() -> StatusCode {
    StatusCode::OK
}
