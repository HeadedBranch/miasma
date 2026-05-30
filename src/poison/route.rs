use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use reqwest::header;
use tokio::sync::OwnedSemaphorePermit;

use super::{LinkSettings, fetch_poison::stream_poison, gzip, response_stream};
use crate::config::MiasmaConfig;

/// Miasma's poison serving trap.
pub async fn serve_poison(
    config: &'static MiasmaConfig,
    in_flight_permit: OwnedSemaphorePermit,
    gzip_response: bool,
    link_settings: LinkSettings<'static>,
) -> impl IntoResponse {
    let poison = match stream_poison(&config.poison_source, config.unsafe_allow_html).await {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error fetching from {}: {e}", config.poison_source);
            // 502 (bad gateway) is the technically correct status code for this case,
            // however, we don't want to leak that we're relying on an upstream
            // service (the poison source). 503 indicates we're temporarily unavailable.
            return StatusCode::SERVICE_UNAVAILABLE.into_response();
        }
    };

    let stream = response_stream::build_response_stream(poison, link_settings, in_flight_permit);

    let body_stream = if gzip_response {
        Body::from_stream(gzip::gzip_stream(stream))
    } else {
        Body::from_stream(stream)
    };

    let mut builder = Response::builder().header(header::CONTENT_TYPE, "text/html");
    if gzip_response {
        builder = builder.header(header::CONTENT_ENCODING, "gzip");
    }
    builder.body(body_stream).unwrap_or_else(|e| {
        eprintln!("Failed to build poison route response: {e}");
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    })
}
