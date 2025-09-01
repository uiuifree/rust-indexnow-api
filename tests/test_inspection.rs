use indexnow_api::IndexNowApi;

#[tokio::test]
async fn test_sitemaps() {
    let api = IndexNowApi::new(
        "www.example.com",
        "452aa38cc3fa4f7ea0893f6b371bc979",
    );
    let a = api.send_urls(vec![
      "https://www.example.com".to_string()

    ]).await;
    println!("{:?}", a);
}
