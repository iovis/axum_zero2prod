#[tokio::test]
async fn health_check_test() {
    spawn_app();

    let client = reqwest::Client::new();
    let response = client
        .get("http://localhost:3000/health_check")
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    tokio::spawn(async move { zero2prod::run().await.unwrap() });
}
