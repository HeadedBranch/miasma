#[cfg(unix)]
use std::fs;

use tokio::net::TcpListener;
#[cfg(unix)]
use tokio::net::UnixListener;

use crate::MiasmaConfig;
use crate::MiasmaError;
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
    /// Create a new Miasma server.
    pub async fn new(config: &'static MiasmaConfig) -> Result<Self, MiasmaError> {
        let listener;

        #[cfg(unix)]
        if let Some(socket) = &config.unix_socket {
            use std::os::unix::fs::PermissionsExt;

            listener =
                Listener::Unix(UnixListener::bind(socket).map_err(MiasmaError::UnixSocketBind)?);
            fs::set_permissions(socket, fs::Permissions::from_mode(0o660))
                .map_err(MiasmaError::UnixSocketBind)?;
        } else {
            let addr = config.address();
            listener = Listener::Tcp(
                TcpListener::bind(&addr)
                    .await
                    .map_err(MiasmaError::TcpBind)?,
            );
        }
        #[cfg(not(unix))]
        {
            let addr = config.address();
            listener = Listener::Tcp(
                TcpListener::bind(&addr)
                    .await
                    .map_err(MiasmaError::TcpBind)?,
            );
        }

        Ok(Self { listener, config })
    }

    /// Start the Miasma server.
    pub async fn run<S>(self, shutdown_signal: S) -> Result<(), MiasmaError>
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
            use colored::Colorize;
            // Add a newline so message does not appear smushed up against '^C' in terminal
            eprintln!("\nFailed to remove {} socket: {e}", socket.blue());
        }

        server_result.map_err(MiasmaError::ServerRuntime)
    }
}
