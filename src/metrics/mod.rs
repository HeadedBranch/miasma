mod error;
mod metrics_counter;
mod page;

use std::sync::Arc;

use axum::{Router, routing::get};
pub use error::MetricsError;
pub use metrics_counter::Metrics;

use tokio::sync::Mutex;

use crate::config::MetricsConfig;

const RESULTS_PER_PAGE: usize = 50;

#[derive(Clone)]
pub struct MetricsState {
    counter: Arc<Mutex<Metrics>>,
    auth_value: Arc<String>,
}

pub fn new_metrics_router(
    config: &Option<MetricsConfig>,
    metrics: Option<Arc<Mutex<Metrics>>>,
) -> Router {
    let Some(config) = config else {
        return Router::new();
    };
    let encoded_creds = config
        .metrics_credentials
        .as_ref()
        .expect("metrics_credentials should be Some")
        .encoded_credentials();
    let auth_value = format!("Basic {encoded_creds}",);

    Router::new()
        .route(&config.metrics_endpoint, get(page::metrics_handler))
        .with_state(MetricsState {
            counter: metrics.expect("metrics should be setup when config is Some"),
            auth_value: Arc::new(auth_value),
        })
}
