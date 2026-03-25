use anyhow::Context;
use colored::Colorize;
use std::sync::LazyLock;

use miasma::{MiasmaConfig, check_for_new_version, new_miasma_router};

// TODO: auto update cargo pacakge version in CD

static CONFIG: LazyLock<MiasmaConfig> = LazyLock::new(MiasmaConfig::new);

fn main() -> anyhow::Result<()> {
    eprintln!("{}\n", "Starting Miasma...".green());

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_name("miasma-thread")
        .build()
        .unwrap()
        .block_on(async {
            let app = new_miasma_router(&CONFIG);

            tokio::spawn(check_for_new_version());

            let addr = format!("{}:{}", CONFIG.host, CONFIG.port);
            let listener = tokio::net::TcpListener::bind(&addr)
                .await
                .with_context(|| format!("could not bind to {addr}").red())?;

            eprintln!(
                "Listening on {} with {} max in-flight requests. Serving poisoned training data from {} with {} links per response...",
                addr.cyan(), CONFIG.max_in_flight.to_string().cyan(),
                CONFIG.poison_source.to_string().cyan(), CONFIG.link_count.to_string().cyan()
            );

            axum::serve(listener, app)
                .await
                .with_context(|| "server exited with an unexpected error".red())
        })
}
