use std::sync::Arc;

use axum::{
    Router,
    body::Body,
    extract::{Query, Request, State, rejection::QueryRejection},
    response::{IntoResponse, Response},
    routing::get,
};
use reqwest::{StatusCode, header};
use serde::Deserialize;
use tokio::sync::{Mutex, Semaphore, TryAcquireError};

use crate::metrics::Metrics;
use crate::poison::{self, LinkSettings};
use crate::{MiasmaConfig, metrics};

#[derive(Deserialize)]
pub struct QueryParams {
    /// We use 'page' instead of depth to look more convincing to scrapers
    page: Option<u32>,
}
impl QueryParams {
    pub const CURRENT_DEPTH_QUERY_PARAM: &str = "page";
}

#[derive(Clone)]
pub struct AppState {
    metrics: Option<Arc<Mutex<Metrics>>>,
    config: &'static MiasmaConfig,
    in_flight_sem: Arc<Semaphore>,
}

/// Build a new `axum::Router` for Miasma's routes.
pub fn new_miasma_router(
    config: &'static MiasmaConfig,
    counter: Option<Arc<Mutex<Metrics>>>,
) -> Router {
    Router::new()
        .fallback(get(app_handler))
        .with_state(AppState {
            config,
            in_flight_sem: Arc::new(Semaphore::new(config.max_in_flight as usize)),
            metrics: counter.clone(),
        })
        .merge(metrics::new_metrics_router(&config.metrics, counter))
}

#[axum::debug_handler(state = AppState)]
async fn app_handler(
    State(state): State<AppState>,
    query: Result<Query<QueryParams>, QueryRejection>,
    req: Request,
) -> impl IntoResponse {
    let in_flight_permit = match state.in_flight_sem.try_acquire_owned() {
        Ok(p) => p,
        Err(TryAcquireError::NoPermits) => {
            return Response::builder()
                .status(StatusCode::TOO_MANY_REQUESTS)
                .header(header::RETRY_AFTER, 5)
                .body(Body::empty())
                .unwrap();
        }
        Err(TryAcquireError::Closed) => {
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let headers = req.headers();

    let gzip_response = state.config.force_gzip
        || headers.get(header::ACCEPT_ENCODING).is_some_and(|acc| {
            acc.to_str()
                .unwrap_or("")
                .split(',')
                // Don't you dare allocate anything !
                .any(|tok| tok.trim().eq_ignore_ascii_case("gzip"))
        });

    let current_depth = query.ok().and_then(|q| q.page).unwrap_or(1);

    if let Some(counter) = state.metrics {
        let user_agent = headers
            .get(header::USER_AGENT)
            .map(|ua| ua.to_str().unwrap_or("INVALID-USER-AGENT-STRING"))
            .unwrap_or("NO-USER-AGENT");
        counter.lock().await.count_request(user_agent);
    }

    let link_settings = LinkSettings::next(state.config, current_depth);

    poison::serve_poison(state.config, in_flight_permit, gzip_response, link_settings)
        .await
        .into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode, header::RETRY_AFTER},
    };
    use std::sync::LazyLock;
    use tower::ServiceExt;

    static TEST_CONFIG: LazyLock<MiasmaConfig> = LazyLock::new(|| MiasmaConfig {
        max_in_flight: 1,
        ..Default::default()
    });

    // This hits example.com over the network; move to an integration test suite eventually.
    #[tokio::test]
    async fn happy_path_works() {
        let app = new_miasma_router(&TEST_CONFIG, None);

        let response = app
            .oneshot(Request::builder().uri("/foo").body(Body::empty()).unwrap())
            .await
            .unwrap();

        // could be 500 if the network is down or 200 if it works, but shouldn't be 429.
        assert_ne!(response.status(), StatusCode::TOO_MANY_REQUESTS);
    }

    #[tokio::test]
    async fn returns_429_when_max_in_flight_reached() {
        let app = new_miasma_router(&TEST_CONFIG, None);
        let req1 = Request::builder().uri("/foo").body(Body::empty()).unwrap();
        let req2 = Request::builder().uri("/foo").body(Body::empty()).unwrap();

        let (res1, res2) = tokio::join!(app.clone().oneshot(req1), app.oneshot(req2));

        let res1 = res1.unwrap();
        let res2 = res2.unwrap();

        let limited = if res1.status() == StatusCode::TOO_MANY_REQUESTS {
            res1
        } else if res2.status() == StatusCode::TOO_MANY_REQUESTS {
            res2
        } else {
            panic!(
                "expected one 429, got {} and {}",
                res1.status(),
                res2.status()
            );
        };

        assert_eq!(limited.status(), StatusCode::TOO_MANY_REQUESTS);
        assert_eq!(
            limited
                .headers()
                .get(RETRY_AFTER)
                .and_then(|v| v.to_str().ok()),
            Some("5")
        );
    }
}
