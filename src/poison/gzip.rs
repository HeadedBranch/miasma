use std::io;

use async_compression::{Level, tokio::bufread::GzipEncoder};
use futures::TryStreamExt;
use tokio::io::BufReader;
use tokio_util::io::{ReaderStream, StreamReader};

use crate::MiasmaStream;

const COMPRESS_BUFFER_SIZE: usize = 1024 * 4;

/// Compresses the poison stream with gzip encoding.
pub fn gzip_stream<E>(stream: impl MiasmaStream<E>) -> impl MiasmaStream<io::Error>
where
    E: Into<anyhow::Error>,
{
    let stream = stream.map_err(|e| io::Error::other(anyhow::anyhow!(e)));
    let reader = StreamReader::new(stream);
    let buf = BufReader::with_capacity(COMPRESS_BUFFER_SIZE, reader);
    let encoder = GzipEncoder::with_quality(
        buf,
        // CRANK IT !
        // we want the smallest response size possible...
        Level::Precise(i32::MAX),
    );
    ReaderStream::new(encoder)
}
