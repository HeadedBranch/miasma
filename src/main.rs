use anyhow::Context;
use axum::routing::get;
use miasma::{MiasmaConfig, routes};

// TODO: add async method to check version and report to user if a newer version can be installed
// TODO: add auto releases/tags as a CD workflow

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = MiasmaConfig::parse();
    let MiasmaConfig {
        port,
        max_connections,
        poison_source,
        links_per_response,
        ..
    } = config.clone();

    let app =
        axum::Router::new().fallback(get(
            move || async move { routes::serve_poison(&config).await },
        ));

    // TODO: 'localhost' may not be the right addr to bind to... do some research
    let listener = tokio::net::TcpListener::bind(format!("localhost:{}", port))
        .await
        .with_context(|| format!("could not bind to port {}", port))?;

    eprintln!(
        "Listening on port {} with {} max connections...",
        port, max_connections
    );
    eprintln!(
        "Serving poisoned training data from {} with {} nested links per response...",
        poison_source, links_per_response
    );

    axum::serve(listener, app)
        .await
        .with_context(|| "server exited with an unexpected error")
}
