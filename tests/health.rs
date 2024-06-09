use simple_newsletter::spawn_app;

#[tokio::test]
async fn health() {
    let host = spawn_app();

    let response = reqwest::get(&format!("{}/health", host))
        .await
        .expect("request failed")
        .error_for_status()
        .expect("non-200 status code");
    println!("{:?}", response);
    assert!(response.status().is_success());
    assert!(response.text().await.unwrap().eq("ok"));
}
