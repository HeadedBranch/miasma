use anyhow::Context;
use colored::Colorize;
use std::sync::LazyLock;

use miasma::{MiasmaConfig, check_for_new_version, new_miasma_router};

static CONFIG: LazyLock<MiasmaConfig> = LazyLock::new(MiasmaConfig::new);

fn main() -> anyhow::Result<()> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_name("miasma-thread")
        .build()
        .unwrap()
        .block_on(async {
            let app = new_miasma_router(&CONFIG);
            eprintln!("{}\n", "Starting Miasma...".green());

            tokio::spawn(check_for_new_version());

            // This logic can probably be improved but it is a quick draft, and the compiler is
            // happy with it, should add #[cfg(unix)] and add err handling
            let addr = CONFIG.address();
            if CONFIG.unix_socket {
                let listener = tokio::net::UnixListener::bind(&addr)
                .with_context(|| format!("Could not bind to {addr}").red())?;
                CONFIG.print_config_info();
                axum::serve(listener, app)
                    .await
                    .with_context(|| "server exited with an unexpected error".red())
            } else {
                let listener = tokio::net::TcpListener::bind(&addr)
                .await
                .with_context(|| format!("could not bind to {addr}").red())?;
                CONFIG.print_config_info();
                axum::serve(listener, app)
                    .await
                    .with_context(|| "server exited with an unexpected error".red())
            }
        })
}
