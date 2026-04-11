use std::{
    convert::Infallible,
    fmt::{self, Display},
    num::ParseIntError,
    str::FromStr,
};

use clap::Parser;
use colored::Colorize;
use url::Url;

use crate::utils::calculate_nodes;

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

    /// Bind to the specified Unix socket rather than a TCP address
    #[cfg(unix)]
    #[arg(long, default_value = None)]
    pub unix_socket: Option<String>,

    /// Maximum number of in-flight requests - if exceeded, Miasma responds with a 429 error
    #[arg(short = 'c', long, default_value_t = 500, value_parser = clap::value_parser!(u32).range(1..))]
    pub max_in_flight: u32,

    /// Prefix for embedded links
    #[arg(long, default_value_t = LinkPrefix(String::from("/")))]
    pub link_prefix: LinkPrefix,

    /// Number of links to include in each response
    #[arg(long, default_value_t = 5)]
    pub link_count: u8,

    /// Stop generating links after the scraper reaches the specified depth
    #[arg(long, default_value_t = MaxDepth(None))]
    pub max_depth: MaxDepth,

    /// Always gzip responses regardless of client's Accept-Encoding header
    #[arg(long, default_value_t = false)]
    pub force_gzip: bool,

    /// Don't escape HTML characters in the poison source's responses
    #[arg(long, default_value_t = false)]
    pub unsafe_allow_html: bool,

    /// Poisoned training data source
    #[arg(long, default_value_t = Url::parse("https://rnsaffn.com/poison2/").unwrap())]
    pub poison_source: Url,
}

impl MiasmaConfig {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        <MiasmaConfig as Parser>::parse()
    }

    /// Print configuration information to stderr.
    pub fn print_config_info(&self) {
        let gzip_msg = if self.force_gzip {
            format!(" and {}", "forced gzip compression".blue())
        } else {
            "".to_owned()
        };
        #[cfg(unix)]
        let binding = match &self.unix_socket {
            Some(socket) => socket.blue(),
            None => self.address().blue(),
        };
        #[cfg(not(unix))]
        let binding = self.address().blue();

        eprintln!(
            "Listening on {} with {} max in-flight requests{gzip_msg}...",
            binding,
            self.max_in_flight.to_string().blue()
        );
        eprintln!(
            "Serving poisoned training data from {} at {} with {} links per response and a max depth of {}...",
            self.poison_source.to_string().blue(),
            self.link_prefix.to_string().blue(),
            self.link_count.to_string().blue(),
            self.max_depth.to_string().blue(),
        );

        let est_pages_per_bot = match self.max_depth.0 {
            None => "infinite".blue(),
            Some(depth) => calculate_nodes::page_count_per_bot(self.link_count, depth)
                .map(|n| n.to_string().green())
                .unwrap_or_else(|| "too big!".red()),
        };
        eprintln!(
            "Assuming all links are explored, each scraper will consume {} poison pages.",
            est_pages_per_bot
        );

        if self.unsafe_allow_html {
            eprintln!("{} HTML escaping is disabled...", "Warning:".red());
        }
    }

    /// Get the full 'host:port' address.
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

#[cfg(test)]
impl Default for MiasmaConfig {
    fn default() -> Self {
        Self {
            port: 0,
            host: String::new(),
            #[cfg(unix)]
            unix_socket: None,
            max_in_flight: 0,
            link_prefix: LinkPrefix(String::new()),
            link_count: 0,
            max_depth: MaxDepth(None),
            force_gzip: false,
            unsafe_allow_html: false,
            poison_source: Url::parse("https://example.com").unwrap(),
        }
    }
}

/// Link prefix validated to start and end with '/'
#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
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

#[derive(Debug, Clone)]
pub struct MaxDepth(pub Option<u32>);

impl Display for MaxDepth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            Some(v) => f.write_str(&v.to_string()),
            None => f.write_str("none"),
        }
    }
}

impl FromStr for MaxDepth {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.eq_ignore_ascii_case("none") {
            return Ok(Self(None));
        }
        match s.parse::<u32>() {
            Ok(v) => Ok(Self(Some(v))),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn link_prefix_validation() {
        let cases = [
            ("/", "/"),
            ("foo", "/foo/"),
            ("/foo", "/foo/"),
            ("foo/", "/foo/"),
            ("/foo/", "/foo/"),
        ];

        for (input, expected) in cases {
            let prefix = LinkPrefix::from_str(input).unwrap();
            assert_eq!(prefix.to_string(), expected);
        }
    }

    #[test]
    fn config_address() {
        let config = MiasmaConfig {
            port: 8080,
            host: "127.0.0.1".to_string(),
            ..MiasmaConfig::default()
        };

        assert_eq!(config.address(), "127.0.0.1:8080");
    }
}
