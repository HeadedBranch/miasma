use std::{
    convert::Infallible,
    fmt::{self, Display},
    str::FromStr,
};

use clap::Parser;
use colored::Colorize;
use url::Url;

/// Config object for miasma.
#[derive(Parser, Debug, Clone)]
#[command(
    version,
    about = "Serve an endless maze of poisoned training data. Fight back against AI web scrapers."
)]
pub struct MiasmaConfig {
    /// Port to listen for requests
    #[arg(short = 'p', long, default_value_t = 9999)]
    pub port: u16,

    /// Host to listen for requests
    #[arg(long, default_value_t = String::from("localhost") )]
    pub host: String,

    /// Maximum number of in-flight requests - if exceeded, Miasma responds with a 429 error
    #[arg(short = 'c', long, default_value_t = 500, value_parser = clap::value_parser!(u32).range(1..))]
    pub max_in_flight: u32,

    /// Prefix for embedded links
    #[arg(long, default_value_t = LinkPrefix(String::from("/")))]
    pub link_prefix: LinkPrefix,

    /// Number of links to include in each response
    #[arg(short = 'l', long, default_value_t = 5)]
    pub link_count: u8,

    /// Always gzip responses regardless of client's Accept-Encoding header
    #[arg(long, default_value_t = false)]
    pub force_gzip: bool,

    /// Don't escape HTML characters in the poison source's responses
    #[arg(long, default_value_t = false)]
    pub unsafe_allow_html: bool,

    /// Poisoned training data source
    #[arg(long, default_value_t = Url::parse("https://rnsaffn.com/poison2/").unwrap())]
    pub poison_source: Url,

    /// The path used to listen for requests, overrides port and host
    #[cfg(unix)]
    #[arg(long, default_value = None)]
    pub unix_socket: Option<String>,
}

impl MiasmaConfig {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        <MiasmaConfig as Parser>::parse()
    }

    /// Print configuration information to stderr.
    pub fn print_config_info(&self) {
        let gzip_msg = if self.force_gzip {
            format!(" and {}", "forced gzip compression".cyan())
        } else {
            "".to_owned()
        };
        eprintln!(
            "Listening on {} with {} max in-flight requests{gzip_msg}...",
            self.address().cyan(),
            self.max_in_flight.to_string().cyan()
        );
        eprintln!(
            "Serving data from {} at {} with {} links per response...",
            self.poison_source.to_string().cyan(),
            self.link_prefix.to_string().cyan(),
            self.link_count.to_string().cyan()
        );
        if self.unsafe_allow_html {
            eprintln!("{} HTML escaping is disabled...", "Warning:".red());
        }
    }

    /// Get the full 'host:port' address.
    #[cfg(unix)]
    pub fn address(&self) -> String {
        if let Some(s) = &self.unix_socket {
            s.to_string()
        } else {
            format!("{}:{}", self.host, self.port)
        }
    }
    #[cfg(not(unix))]
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
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
