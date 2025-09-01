use indexnow_api::IndexNowApi;

#[tokio::test]
async fn test_sitemaps() {
    let api = IndexNowApi::new(
        "www.example.com",
        "7be9fca90b3b4b039983fa8f06e03ee8",
    );
    let a = api.send_urls(vec![
      "https://www.example.com".to_string()

    ]).await;
    println!("{:?}", a);
}
