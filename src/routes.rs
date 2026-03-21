mod html_builder;
mod poison;

use axum::{http::StatusCode, response::Html};

use crate::MiasmaConfig;

// TODO: implement max connections & return 429 if saturated
// TODO: stream resonse rather than creating HTML struct
// TODO: compress response to save on bandwith costs

/// Miasma's poison serving trap.
pub async fn serve_poison(config: &MiasmaConfig) -> (StatusCode, Html<String>) {
    let poison = poison::fetch_poison(&config.poison_source)
        .await
        .inspect_err(|e| eprintln!("Error fetching from poison source: {e}"))
        .unwrap_or_else(|e| format!("Hmmm, something went wrong here\n{e}"));
    let page =
        html_builder::build_html_str(config.links_per_response, &config.link_prefix, &poison).await;
    (StatusCode::OK, Html(page))
}
