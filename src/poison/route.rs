use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use reqwest::header;
use tokio::sync::OwnedSemaphorePermit;

use super::{LinkSettings, fetch_poison::stream_poison, gzip, html_builder};
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
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let stream =
        html_builder::POISON_PAGE.build_html_stream(poison, link_settings, in_flight_permit);

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
