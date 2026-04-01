use anyhow::Context;
use colored::Colorize;
use std::sync::LazyLock;
use tokio::net::{TcpListener};
#[cfg(unix)]
use tokio::net::{UnixListener, UnixStream, TcpStream};
#[cfg(unix)]
use tokio_util::either::Either;
use tokio::signal::ctrl_c;

use miasma::{MiasmaConfig, check_for_new_version, new_miasma_router};

static CONFIG: LazyLock<MiasmaConfig> = LazyLock::new(MiasmaConfig::new);

// This is extremely ugly but it does work, should probably be moved somewhere
// This allows both TcpListener and UnixListener to be used by axum::serve
// It does this by implementing the trait it requires on a custom type
// (needed because of orphan rules), I know it looks ugly I wrote it
#[cfg(unix)]
struct Listeners (Either<TcpListener, UnixListener>);
#[cfg(unix)]
impl axum::serve::Listener for Listeners {
    type Io = Either<TcpStream, UnixStream>;
    type Addr = Either<std::net::SocketAddr, tokio::net::unix::SocketAddr>;

    async fn accept(&mut self) -> (Self::Io, Self::Addr) {
        match &self.0 {
            Either::Left(l) => {
                let (stream, addr) = l.accept().await.unwrap();
                (Either::Left(stream), Either::Left(addr))
            }
            Either::Right(r) => {
                let (stream, addr) = r.accept().await.unwrap();
                (Either::Right(stream), Either::Right(addr))
            }
        }
    }
    fn local_addr(&self) -> Result<Self::Addr, std::io::Error> {
        match &self.0 {
            Either::Left(l) => Ok(Either::Left(l.local_addr()?)),
            Either::Right(r) => Ok(Either::Right(r.local_addr()?)),
        }
    }
}

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

            let addr = CONFIG.address();
            #[cfg(unix)]
            let listener = if CONFIG.unix_socket {
                Listeners(Either::Right(UnixListener::bind(&addr)
                    .with_context(|| format!("Could not bind to {addr}").red())?))
            } else {
                Listeners(Either::Left(TcpListener::bind(&addr)
                    .await
                    .with_context(|| format!("could not bind to {addr}").red())?))
            };
            #[cfg(not(unix))]
            let listener = TcpListener::bind(&addr)
                .await
                .with_context(|| format!("could not bind to {addr}").red())?;


            CONFIG.print_config_info();

            tokio::select! {
                _ = ctrl_c() => {
                    #[cfg(unix)]
                    if CONFIG.unix_socket {
                        if let Err(e) = std::fs::remove_file(CONFIG.host.clone()) {
                            println!("Error {e} removing {}, you may need to delete it manually", CONFIG.host.cyan());
                        }
                    }
                    Ok(())
                }

                main_runner = axum::serve(listener, app) => {
                        main_runner.with_context(|| "server exited with an unexpected error".red())
                    }
            }
        })
}
