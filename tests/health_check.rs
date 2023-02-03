#[tokio::test]
async fn health_check_works(){
    spawn_app().await.expect("Failed to spawn our app.");
    // perform http request against our app.
    let client = reqwest::Client::new();

    let response = client
        .get("http://127.0.0.1:8080/health_check")
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(response.status().is_success());
}

async fn spawn_app() -> std::io::Result<()> {
    zero2prod::run().await
}
