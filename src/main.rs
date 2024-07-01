use sqlx::SqlitePool;
use zero2prod::configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = configuration::get().expect("Failed to read configuration");
    let db = SqlitePool::connect(&config.database.name)
        .await
        .expect("Failed to connect to DB");

    let addr = format!("127.0.0.1:{}", config.application_port);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    run(listener, db).await
}
