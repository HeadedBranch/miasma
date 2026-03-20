use reqwest::Client;
use std::{sync::LazyLock, time::Duration};
use url::Url;

static CLIENT: LazyLock<Client> = LazyLock::new(|| {
    reqwest::Client::builder()
        .gzip(true) // Poison Fountain serves gzipped data
        .timeout(Duration::from_secs(5))
        .user_agent(concat!(
            env!("CARGO_PKG_NAME"),
            "/",
            env!("CARGO_PKG_VERSION")
        ))
        .build()
        .expect("should be able to build client")
});

pub async fn fetch_poison(poison_source: &Url) -> anyhow::Result<String> {
    let text = CLIENT
        .get(poison_source.clone())
        .send()
        .await?
        .text()
        .await?;
    // TODO: escape HTML sequences in the poison string
    // It's possible that the poison source will send JavaScript within `<script>` tags, which will
    // execute in browsers.
    // Probably a very niche case we shouldn't worry about, but worth documenting...
    Ok(text)
}
