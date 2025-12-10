use crate::{test_config, BrowserConfig};

#[tokio::test]
async fn test_config_disable_https_first() {
    test_config(
        BrowserConfig::builder()
            .no_sandbox()
            .disable_https_first()
            .build()
            .unwrap(),
        async |browser| {
            let page = browser.new_page("about:blank").await.unwrap();
            page.goto("http://neverssl.com").await.unwrap();
            let url = page.url().await.unwrap().unwrap();
            assert!(url.starts_with("http://"));
        },
    )
    .await;
}
