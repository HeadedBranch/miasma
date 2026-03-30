use anyhow::Context;
use colored::Colorize;
use std::sync::LazyLock;

use miasma::{MiasmaConfig, check_for_new_version, new_miasma_router};

static CONFIG: LazyLock<MiasmaConfig> = LazyLock::new(MiasmaConfig::new);

// TODO: randomize html template content
// TODO: improve test coverage

fn main() -> anyhow::Result<()> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_name("miasma-thread")
        .build()
        .unwrap()
        .block_on(async {
            let app = new_miasma_router(&CONFIG);
            eprintln!("{}\n", "Starting Miasma...".green());

            //tokio::spawn(check_for_new_version());

            // This logic can probably be improved but it is a quick draft, and the compiler is
            // happy with it, should add #[cfg(unix)] and add err handling
            let is_unix_socket = CONFIG.host.starts_with("unix://");
            if  is_unix_socket {
                let addr = format!("{}", CONFIG.host.strip_prefix("unix://").unwrap());
                let listener = tokio::net::UnixListener::bind(&addr)
                .with_context(|| format!("Could not bind to {addr}").red())?;
                eprintln!(
                    "Listening on {} with {} max in-flight requests. Serving poisoned training data from {} with {} links per response...",
                    addr.cyan(), CONFIG.max_in_flight.to_string().cyan(),
                    CONFIG.poison_source.to_string().cyan(), CONFIG.link_count.to_string().cyan()
                );
                axum::serve(listener, app)
                    .await
                    .with_context(|| "server exited with an unexpected error".red())
            } else {
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
            }


        })
}
