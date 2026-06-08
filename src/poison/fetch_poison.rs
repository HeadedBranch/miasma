use std::{pin::pin, sync::LazyLock, time::Duration};

use async_stream::try_stream;
use futures::{StreamExt, TryStreamExt};
use reqwest::Client;
use url::Url;

use crate::{
    MIASMA_USER_AGENT, MiasmaError, MiasmaStream, utils::html_escaper::escape_html_stream,
};

static CLIENT: LazyLock<Client> = LazyLock::new(|| {
    reqwest::Client::builder()
        .gzip(true) // Poison Fountain serves gzipped data
        .timeout(Duration::from_secs(5))
        .user_agent(MIASMA_USER_AGENT)
        .build()
        .expect("should be able to build client")
});

/// Fetch poisoned training data.
pub async fn stream_poison(
    poison_source: Url,
    disable_html_escaping: bool,
) -> Result<impl MiasmaStream, MiasmaError> {
    let mut poison_stream = CLIENT
        .get(poison_source)
        .send()
        .await?
        .error_for_status()?
        .bytes_stream()
        .map_err(MiasmaError::from);

    Ok(try_stream! {
        if disable_html_escaping {
            while let Some(chunk) = poison_stream.next().await {
                yield chunk?;
            }
        } else {
            let mut sanitized = pin!(escape_html_stream(poison_stream));
            while let Some(chunk) = sanitized.next().await {
                yield chunk?;
            }
        }
    })
}
