#[cfg(unix)]
use std::fs;

use tokio::net::TcpListener;
#[cfg(unix)]
use tokio::net::UnixListener;

use miasma::new_miasma_router;
use miasma::{MiasmaConfig, MiasmaError};

use crate::cli::AppArgs;

enum Listener {
    Tcp(TcpListener),
    #[cfg(unix)]
    Unix(UnixListener),
}

pub struct App {
    listener: Listener,
    config: MiasmaConfig,
    #[cfg(unix)]
    socket: Option<String>,
}

impl App {
    /// Create a new Miasma server.
    pub async fn new(args: AppArgs) -> Result<Self, MiasmaError> {
        let listener;

        #[cfg(unix)]
        if let Some(socket) = &args.unix_socket {
            use std::os::unix::fs::PermissionsExt;

            listener =
                Listener::Unix(UnixListener::bind(socket).map_err(MiasmaError::UnixSocketBind)?);
            fs::set_permissions(socket, fs::Permissions::from_mode(0o660))
                .map_err(MiasmaError::UnixSocketBind)?;
        } else {
            let addr = args.address();
            listener = Listener::Tcp(
                TcpListener::bind(&addr)
                    .await
                    .map_err(MiasmaError::TcpBind)?,
            );
        }
        #[cfg(not(unix))]
        {
            let addr = args.address();
            listener = Listener::Tcp(
                TcpListener::bind(&addr)
                    .await
                    .map_err(MiasmaError::TcpBind)?,
            );
        }

        Ok(Self {
            listener,
            config: args.to_miasma_config(),
            #[cfg(unix)]
            socket: args.unix_socket,
        })
    }

    /// Start the Miasma server.
    pub async fn run<S>(self, shutdown_signal: S) -> Result<(), MiasmaError>
    where
        S: Future<Output = ()> + Send + 'static,
    {
        let router = new_miasma_router(self.config)?;

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
        if let Some(socket) = &self.socket
            && let Err(e) = fs::remove_file(socket)
        {
            use colored::Colorize;
            // Add a newline so message does not appear smushed up against '^C' in terminal
            eprintln!("\nFailed to remove {} socket: {e}", socket.blue());
        }

        server_result.map_err(MiasmaError::ServerRuntime)
    }
}
