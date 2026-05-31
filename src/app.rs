#[cfg(unix)]
use std::fs;

use std::sync::Arc;
use tokio::net::TcpListener;
#[cfg(unix)]
use tokio::net::UnixListener;
use tokio::sync::Mutex;

use crate::MiasmaConfig;
use crate::MiasmaError;
use crate::new_miasma_router;

use crate::metrics::Metrics;

enum Listener {
    Tcp(TcpListener),
    #[cfg(unix)]
    Unix(UnixListener),
}

pub struct Miasma {
    listener: Listener,
    config: &'static MiasmaConfig,
    metrics: Option<Arc<Mutex<Metrics>>>,
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

        let metrics = if let Some(metrics_config) = &config.metrics {
            let metrics = Metrics::new(
                metrics_config
                    .metrics_db_path
                    .clone()
                    .expect("metrics_db_path should be set if metrics is Some"),
            )?;
            Some(Arc::new(Mutex::new(metrics)))
        } else {
            None
        };

        Ok(Self {
            listener,
            config,
            metrics,
        })
    }

    /// Start the Miasma server.
    pub async fn run<S>(self, shutdown_signal: S) -> Result<(), MiasmaError>
    where
        S: Future<Output = ()> + Send + 'static,
    {
        let router = new_miasma_router(self.config, self.metrics.clone());

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

        if let Some(metrics) = self.metrics {
            match metrics.try_lock() {
                Ok(mut l) => l.flush_blocking(),
                Err(e) => eprintln!("failed to get metrics lock: {e}"),
            }
        }

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
