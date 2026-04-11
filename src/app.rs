#[cfg(unix)]
use std::fs;

use anyhow::Context;
use colored::Colorize;
use tokio::net::TcpListener;
#[cfg(unix)]
use tokio::net::UnixListener;

use crate::MiasmaConfig;
use crate::new_miasma_router;

enum Listener {
    Tcp(TcpListener),
    #[cfg(unix)]
    Unix(UnixListener),
}

pub struct Miasma {
    listener: Listener,
    config: &'static MiasmaConfig,
}

impl Miasma {
    // TODO: return a custom error type here rather than anyhow::Error
    /// Create a new Miasma server.
    pub async fn new(config: &'static MiasmaConfig) -> anyhow::Result<Self> {
        let listener;

        #[cfg(unix)]
        if let Some(socket) = &config.unix_socket {
            listener = Listener::Unix(
                UnixListener::bind(socket)
                    .with_context(|| format!("could not bind to {socket}").red())?,
            );
        } else {
            let addr = config.address();
            listener = Listener::Tcp(
                TcpListener::bind(&addr)
                    .await
                    .with_context(|| format!("could not bind to {addr}").red())?,
            );
        }
        #[cfg(not(unix))]
        {
            let addr = config.address();
            listener = Listener::Tcp(
                TcpListener::bind(&addr)
                    .await
                    .with_context(|| format!("could not bind to {addr}").red())?,
            );
        }

        Ok(Self { listener, config })
    }

    /// Start the Miasma server.
    pub async fn run<S>(self, shutdown_signal: S) -> Result<(), anyhow::Error>
    where
        S: Future<Output = ()> + Send + 'static,
    {
        let router = new_miasma_router(self.config);

        let server_result = match self.listener {
            Listener::Tcp(tcp) => {
                axum::serve(tcp, router)
                    .with_graceful_shutdown(shutdown_signal)
                    .await
            }
            #[cfg(unix)]
            Listener::Unix(unix) => {
                axum::serve(unix, router)
                    .with_graceful_shutdown(shutdown_signal)
                    .await
            }
        };

        #[cfg(unix)]
        if let Some(socket) = &self.config.unix_socket
            && let Err(e) = fs::remove_file(socket)
        {
            // Add a newline so message does not appear smushed up against '^C' in terminal
            eprintln!("\nFailed to remove {} socket: {e}", socket.blue());
        }

        server_result.with_context(|| "server exited with an unexpected error".red())
    }
}
