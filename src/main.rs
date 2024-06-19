use axum::http::StatusCode;
use axum::{extract::Path, routing::get, Router};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/health_check", get(health_check))
        .route("/:name", get(greet));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn greet(Path(name): Path<String>) -> String {
    format!("Hello, {name}!")
}

#[axum::debug_handler]
async fn health_check() -> StatusCode {
    StatusCode::OK
}
