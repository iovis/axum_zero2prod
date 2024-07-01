use axum::routing::{get, post};
use axum::Router;
use sqlx::SqlitePool;
use tokio::net::TcpListener;

use crate::routes::{health_check, subscribe};

pub async fn run(listener: TcpListener, db: SqlitePool) -> Result<(), std::io::Error> {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .with_state(db);

    axum::serve(listener, app).await
}
