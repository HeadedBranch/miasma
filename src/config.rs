use std::{
    convert::Infallible,
    fmt::{self, Display},
    str::FromStr,
};

use clap::Parser;
use url::Url;

/// Config object for miasma.
#[derive(Parser, Debug, Clone)]
#[command(
    version,
    about = "Serve an endless maze of poisoned training data. Fight back against AI web scrapers."
)]
pub struct MiasmaConfig {
    /// port to listen for requests
    #[arg(short = 'p', long, default_value_t = 9999)]
    pub port: u16,

    /// host to listen for requests
    #[arg(long, default_value_t = String::from("localhost") )]
    pub host: String,

    /// maximum number of in-flight requests - if exceeded, miasma responds with a 429 error
    #[arg(short = 'c', long, default_value_t = 500, value_parser = clap::value_parser!(u32).range(1..))]
    pub max_in_flight: u32,

    /// number of links to include in each response
    #[arg(short = 'l', long, default_value_t = 5)]
    pub link_count: u8,

    /// prefix for embedded links
    #[arg(long, default_value_t = LinkPrefix(String::from("/")))]
    pub link_prefix: LinkPrefix,

    /// poisoned training data source
    #[arg(long, default_value_t = Url::parse("https://rnsaffn.com/poison2/").unwrap())]
    pub poison_source: Url,
}

impl MiasmaConfig {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        <MiasmaConfig as Parser>::parse()
    }
}

/// Link prefix validated to start and end with '/'
#[derive(Debug, Clone)]
pub struct LinkPrefix(String);

impl Display for LinkPrefix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for LinkPrefix {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut prefix = s.to_owned();
        if !prefix.starts_with('/') {
            prefix.insert(0, '/');
        }
        if !prefix.ends_with('/') {
            prefix.push('/');
        }
        Ok(Self(prefix))
    }
}
