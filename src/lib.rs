mod app;
mod config;
mod poison;
mod router;
mod utils;
mod version_check;

use router::QueryParams;
use router::new_miasma_router;

pub use app::Miasma;
pub use config::MiasmaConfig;
pub use version_check::check_for_new_version;

use bytes::Bytes;
use futures::Stream;

const USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " (github.com/austin-weeks/miasma)"
);

/// Alias for Stream of `Result<Bytes, E>`
pub trait MiasmaStream<E = anyhow::Error>: Stream<Item = Result<Bytes, E>> {}
impl<T, E> MiasmaStream<E> for T where T: Stream<Item = Result<Bytes, E>> {}
