#[tokio::test]
async fn health_check_test() {
    let url = spawn_app().await;

    let response = reqwest::Client::new()
        .get(format!("{url}/health_check"))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() -> String {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();

    tokio::spawn(async move { zero2prod::run(listener).await.unwrap() });

    format!("http://127.0.0.1:{port}")
}
