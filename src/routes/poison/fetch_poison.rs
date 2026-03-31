use std::{pin::pin, sync::LazyLock, time::Duration};

use async_stream::stream;
use bytes::Bytes;
use futures::{Stream, StreamExt, TryStreamExt};
use reqwest::Client;
use url::Url;

use crate::{USER_AGENT, utils::html_escaper::escape_html_stream};

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
    disable_html_escaping: bool,
) -> Result<impl Stream<Item = Result<Bytes, anyhow::Error>>, anyhow::Error> {
    let mut poison_stream = CLIENT
        .get(poison_source.clone())
        .send()
        .await?
        .error_for_status()?
        .bytes_stream()
        .map_err(anyhow::Error::from);

    Ok(stream! {
        if disable_html_escaping {
            while let Some(chunk) = poison_stream.next().await {
                yield chunk;
            }
        } else {
            let mut sanitized = pin!(escape_html_stream(poison_stream));
            while let Some(chunk) = sanitized.next().await {
                yield chunk;
            }
        }
    })
}
