use anyhow::Context;
use colored::Colorize;
use std::sync::LazyLock;
use tokio::net::{UnixListener, TcpListener, UnixStream, TcpStream};
use tokio_util::either::Either;

use miasma::{MiasmaConfig, check_for_new_version, new_miasma_router};

static CONFIG: LazyLock<MiasmaConfig> = LazyLock::new(MiasmaConfig::new);

// This is extremely ugly but it does work, should probably be moved somewhere
// This allows both TcpListener and UnixListener to be used by axum::serve
// It does this by implementing the trait it requires on a custom type
// (needed because of orphan rules), I know it looks ugly I wrote it
struct Listeners (Either<TcpListener, UnixListener>);

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
            let listener = if cfg!(unix) && CONFIG.unix_socket {
                Listeners(Either::Right(UnixListener::bind(&addr)
                    .with_context(|| format!("Could not bind to {addr}").red())?))
            } else {
                Listeners(Either::Left(TcpListener::bind(&addr)
                    .await
                    .with_context(|| format!("could not bind to {addr}").red())?))
            };

            CONFIG.print_config_info();

            axum::serve(listener, app)
                .await
                .with_context(|| "server exited with an unexpected error".red())
        })
}
