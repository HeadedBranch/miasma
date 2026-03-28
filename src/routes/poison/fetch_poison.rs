use bytes::Bytes;
use futures::Stream;
use reqwest::{Client, Error};
use std::{sync::LazyLock, time::Duration};
use url::Url;

use crate::USER_AGENT;

static CLIENT: LazyLock<Client> = LazyLock::new(|| {
    reqwest::Client::builder()
        .gzip(true) // Poison Fountain serves gzipped data
        .timeout(Duration::from_secs(5))
        .user_agent(USER_AGENT)
        .build()
        .expect("should be able to build client")
});

/// Fetch poisoned training data.
pub async fn stream_poison(
    poison_source: &Url,
) -> Result<impl Stream<Item = Result<Bytes, Error>>, Error> {
    let stream = CLIENT
        .get(poison_source.clone())
        .send()
        .await?
        .error_for_status()?
        .bytes_stream();

    // NOTE: It's possible that the poison source will send JavaScript within `<script>` tags,
    // which will execute in browsers.
    // This is a very niche case we probably shouldn't worry about, but worth documenting...
    Ok(stream)
}
