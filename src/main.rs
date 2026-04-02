use anyhow::Context;
use colored::Colorize;
use std::sync::LazyLock;
use tokio::net::TcpListener;
#[cfg(unix)]
use tokio::net::UnixListener;

use miasma::{check_for_new_version, new_miasma_router, MiasmaConfig};

static CONFIG: LazyLock<MiasmaConfig> = LazyLock::new(MiasmaConfig::new);

fn main() -> anyhow::Result<()> {
    let app_result = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_name("miasma-thread")
        .build()
        .unwrap()
        .block_on(async {
            let app = new_miasma_router(&CONFIG);
            eprintln!("{}\n", "Starting Miasma...".green());

            tokio::spawn(check_for_new_version());

            let addr = CONFIG.address();
            #[cfg(unix)]
            if let Some(s) = &CONFIG.unix_socket {
                let listener = UnixListener::bind(s)
                    .with_context(|| format!("Could not bind to {s}").red())?;
                CONFIG.print_config_info();

                axum::serve(listener, app).await.with_context(|| "server exited with an unexpected error".red())

            } else {
                let listener = TcpListener::bind(&addr)
                    .await
                    .with_context(|| format!("could not bind to {addr}").red())?;

                CONFIG.print_config_info();

                axum::serve(listener, app).await.with_context(|| "server exited with an unexpected error".red())
            }
            #[cfg(not(unix))]
            {
                let listener = TcpListener::bind(&addr)
                    .await
                    .with_context(|| format!("could not bind to {addr}").red())?;

                CONFIG.print_config_info();

                axum::serve(listener, app).await.with_context(|| "server exited with an unexpected error".red())
            }
        });

    if let Some(path) = &CONFIG.unix_socket {
        if let Err(e) = std::fs::remove_file(path) {
            println!("Error {e} removing {}, you may need to delete it manually", CONFIG.host.cyan());
        }
    }
    app_result
}
