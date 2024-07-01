use sqlx::SqlitePool;
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub db: SqlitePool,
}

async fn spawn_app() -> TestApp {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();

    // `mode=rwc` => create if missing
    let db_name = format!("sqlite://db/test-{}.db?mode=rwc", Uuid::new_v4());
    let db = configure_database(&db_name).await;

    let server = zero2prod::startup::run(listener, db.clone());
    tokio::spawn(server);

    TestApp {
        address: format!("http://127.0.0.1:{port}"),
        db,
    }
}

pub async fn configure_database(name: &str) -> SqlitePool {
    let db = SqlitePool::connect(name)
        .await
        .expect("Failed to connect to create {name}");

    sqlx::migrate!("./migrations")
        .run(&db)
        .await
        .expect("Failed to migrate {name}");

    db
}

#[tokio::test]
async fn health_check_test() {
    let app = spawn_app().await;

    let response = reqwest::Client::new()
        .get(format!("{}/health_check", app.address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data_test() {
    let app = spawn_app().await;

    let body = "name=Dabidu Fartchante&email=dabidu.fartchante@gmail.com";

    let response = reqwest::Client::new()
        .post(format!("{}/subscriptions", app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status().as_u16(), 200);

    let saved = sqlx::query!("select name, email from subscriptions",)
        .fetch_one(&app.db)
        .await
        .expect("Failed to fetch subscription");

    assert_eq!(saved.name, "Dabidu Fartchante");
    assert_eq!(saved.email, "dabidu.fartchante@gmail.com");
}

#[tokio::test]
async fn subscribe_returns_400_for_when_data_is_missing_test() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=Davidu Fartchante", "missing email"),
        ("email=dabidu.fartchante@gmail.com", "missing name"),
        ("", "missing name and email"),
    ];

    for (body, message) in test_cases {
        let response = client
            .post(format!("{}/subscriptions", app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed running scenario: {message}"
        );
    }
}
