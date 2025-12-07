use crate::{test_config, BrowserConfig};

#[tokio::test]
async fn test_config_disable_https_first() {
    test_config(
        BrowserConfig::builder()
            .disable_https_first()
            .build()
            .unwrap(),
        async |browser| {
            let page = browser.new_page("about:blank").await.unwrap();
            page.goto("http://perdu.com").await.unwrap();
            let title = page.url().await.unwrap().unwrap();
            assert!(title.starts_with("http://"));
        },
    )
    .await;
}
