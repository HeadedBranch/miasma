use std::{
    fmt::{self, Display},
    num::ParseIntError,
    str::FromStr,
};

use crate::cli;
use clap::{Args, Parser};
use colored::Colorize;
use miasma::MiasmaConfig;
use url::Url;

#[derive(Parser, Debug, Clone)]
#[command(
    version,
    about = "Serve an endless maze of poisoned training data. Fight back against AI web scrapers."
)]
pub struct AppArgs {
    /// Port to listen for requests
    #[arg(short = 'p', long, default_value_t = 9999)]
    #[arg(help_heading = "Server Options")]
    pub port: u16,

    /// Host to listen for requests
    #[arg(long, env = "MIASMA_HOST", default_value_t = String::from("localhost") )]
    #[arg(help_heading = "Server Options")]
    pub host: String,

    /// Bind to the specified Unix socket rather than a TCP address
    #[cfg(unix)]
    #[arg(long, default_value = None)]
    #[arg(help_heading = "Server Options")]
    pub unix_socket: Option<String>,

    /// Maximum number of in-flight requests - if exceeded, Miasma responds with a 429 error
    #[arg(short = 'c', long, default_value_t = miasma::DEFAULT_MAX_IN_FLIGHT, value_parser = clap::value_parser!(u32).range(1..))]
    pub max_in_flight: u32,

    /// Prefix for embedded links
    #[arg(long, default_value_t = String::from("/"))]
    pub link_prefix: String,

    /// Number of links to include in each response
    #[arg(long, default_value_t = miasma::DEFAULT_LINK_COUNT)]
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
    #[arg(long, default_value_t = Url::parse(miasma::DEFAULT_POISON_SOURCE).unwrap())]
    pub poison_source: Url,

    #[command(flatten)]
    pub metrics: Option<MetricsConfig>,
}

impl AppArgs {
    pub fn to_miasma_config(&self) -> MiasmaConfig {
        let mut builder = MiasmaConfig::builder();
        builder
            .max_in_flight(self.max_in_flight)
            .link_count(self.link_count)
            .link_prefix(&self.link_prefix)
            .force_gzip(self.force_gzip)
            .unsafe_allow_html(self.unsafe_allow_html)
            .poison_source(self.poison_source.clone());

        if let Some(d) = self.max_depth.0 {
            builder.max_depth(d);
        }
        if let Some(m) = &self.metrics {
            let credentials = m
                .metrics_credentials
                .as_ref()
                .expect("credentials should be Some if MetricsConfig is Some");
            builder.metrics(
                m.metrics_db_path
                    .as_ref()
                    .expect("db path should be Some if MetricsConfig is Some"),
                &m.metrics_endpoint,
                &credentials.username,
                &credentials.password,
            );
        }

        builder.build()
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

/// If this value is Some, all sub-fields will also be Some
#[derive(Args, Debug, Clone)]
#[allow(clippy::struct_field_names)]
pub struct MetricsConfig {
    #[allow(clippy::doc_markdown)]
    /// Path to SQLite database file (e.g. 'miasma.db')
    #[arg(long, requires = "metrics_credentials")]
    #[arg(help_heading = "Metrics Options")]
    pub metrics_db_path: Option<String>,

    /// Basic auth credentials required to access request metrics -
    /// must match format '<username>:<password>'
    #[arg(long, requires = "metrics_db_path")]
    #[arg(help_heading = "Metrics Options")]
    pub metrics_credentials: Option<MetricsCredentials>,

    /// Endpoint at which request metrics will be served
    #[arg(
        long, default_value = "/metrics",
        requires_all = ["metrics_db_path", "metrics_credentials"],
    )]
    #[arg(help_heading = "Metrics Options")]
    pub metrics_endpoint: String,
}

#[derive(Debug, Clone)]
pub struct MetricsCredentials {
    username: String,
    password: String,
}

impl Display for MetricsCredentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}:******", self.username))
    }
}

impl FromStr for MetricsCredentials {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((username, password)) = s.split_once(':') else {
            return Err("credentials must match format '<username>:<password>'");
        };
        if username.is_empty() {
            return Err("username must not be empty");
        }
        if password.is_empty() {
            return Err("password must not be empty");
        }
        Ok(Self {
            username: username.to_owned(),
            password: password.to_owned(),
        })
    }
}

impl AppArgs {
    pub fn parse_args() -> Self {
        <AppArgs as Parser>::parse()
    }

    /// Print configuration information to stderr.
    pub fn print_config_info(&self) {
        let gzip_msg = if self.force_gzip {
            format!(" and {}", "forced gzip compression".blue())
        } else {
            String::new()
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
            self.link_prefix.blue(),
            self.link_count.to_string().blue(),
            self.max_depth.to_string().blue(),
        );

        let est_pages_per_bot = match self.max_depth.0 {
            None => "infinite".blue(),
            Some(depth) => cli::page_count_per_bot(self.link_count, depth)
                .map_or_else(|| "too big!".red(), |n| n.to_string().green()),
        };
        eprintln!(
            "Assuming all links are explored, each scraper will consume {est_pages_per_bot} poison pages."
        );

        if self.unsafe_allow_html {
            eprintln!("{} HTML escaping is disabled...", "Warning:".red());
        }

        if let Some(metrics) = &self.metrics {
            eprintln!(
                "Request metrics are available at {} with credentials {}.",
                metrics.metrics_endpoint.blue(),
                metrics
                    .metrics_credentials
                    .as_ref()
                    .unwrap()
                    .to_string()
                    .blue(),
            );
            eprintln!(
                "Metrics will be written to the SQLite database at {}.",
                metrics.metrics_db_path.as_ref().unwrap().blue(),
            );
        } else {
            eprintln!("Metrics are disabled and will not be collected...");
        }
    }

    /// Get the full 'host:port' address.
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use base64::prelude::*;

    #[test]
    fn to_miasma_config() {
        let poison_source = Url::parse("https://example.com").unwrap();
        let args = AppArgs {
            #[cfg(unix)]
            unix_socket: None,
            port: 0,
            host: String::new(),

            max_in_flight: 8,
            link_prefix: "/bots".to_owned(),
            link_count: 8,
            max_depth: MaxDepth(Some(8)),
            poison_source: poison_source.clone(),
            force_gzip: true,
            unsafe_allow_html: true,

            metrics: Some(MetricsConfig {
                metrics_db_path: Some("miasma.db".to_owned()),
                metrics_credentials: Some(MetricsCredentials {
                    username: "admin".to_owned(),
                    password: "admin".to_owned(),
                }),
                metrics_endpoint: "/serve-metrics".to_owned(),
            }),
        };

        let config = args.to_miasma_config();

        assert_eq!(config.max_in_flight, 8);
        assert_eq!(config.link_prefix.to_string(), "/bots/".to_owned());
        assert_eq!(config.link_count, 8);
        assert_eq!(config.max_depth, Some(8));
        assert_eq!(config.poison_source, poison_source);
        assert!(config.force_gzip);
        assert!(config.unsafe_allow_html);

        let metrics = config.metrics.unwrap();
        assert_eq!(metrics.db_path, "miasma.db".to_owned());
        assert_eq!(metrics.endpoint, "/serve-metrics".to_owned());
        assert_eq!(
            metrics.encoded_credentials(),
            BASE64_STANDARD.encode("admin:admin")
        );
    }

    #[test]
    fn args_address() {
        let config = AppArgs {
            #[cfg(unix)]
            unix_socket: None,
            port: 8080,
            host: "127.0.0.1".to_owned(),
            max_in_flight: Default::default(),
            link_prefix: String::default(),
            link_count: Default::default(),
            force_gzip: Default::default(),
            unsafe_allow_html: Default::default(),
            metrics: None,
            max_depth: MaxDepth(None),
            poison_source: Url::parse("https://example.com").unwrap(),
        };

        assert_eq!(config.address(), "127.0.0.1:8080");
    }

    #[test]
    fn metrics_credentials_validation() {
        let cases = [
            ("", false, ("", "")),
            ("usernamepassword", false, ("", "")),
            (":password", false, ("", "")),
            ("username:", false, ("", "")),
            ("username:password", true, ("username", "password")),
        ];

        for (input, valid, (uname, pword)) in cases {
            match MetricsCredentials::from_str(input) {
                Err(_) => assert!(!valid),
                Ok(MetricsCredentials { username, password }) => {
                    assert_eq!(username, uname);
                    assert_eq!(password, pword);
                }
            }
        }
    }
}
