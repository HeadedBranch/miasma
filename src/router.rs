use std::sync::Arc;

use axum::{
    Router,
    body::Body,
    http::Request,
    response::{IntoResponse, Response},
    routing::get,
};
use reqwest::{StatusCode, header};
use serde::Deserialize;
use tokio::sync::{Semaphore, TryAcquireError};

use crate::MiasmaConfig;
use crate::poison::{self, LinkSettings};

#[derive(Deserialize)]
pub struct QueryParams {
    /// We use 'page' instead of depth to look more convincing to scrapers
    page: Option<u32>,
}
impl QueryParams {
    pub const CURRENT_DEPTH_QUERY_PARAM: &str = "page";
}

/// Build a new `axum::Router` for Miasma's routes.
pub fn new_miasma_router(config: &'static MiasmaConfig) -> Router {
    let in_flight_sem = Arc::new(Semaphore::new(config.max_in_flight as usize));

    Router::new().fallback(get(move |req: Request<Body>| async move {
        let in_flight_permit = match in_flight_sem.try_acquire_owned() {
            Ok(p) => p,
            Err(e) => match e {
                TryAcquireError::NoPermits => {
                    return Response::builder()
                        .status(StatusCode::TOO_MANY_REQUESTS)
                        .header(header::RETRY_AFTER, 5)
                        .body(Body::empty())
                        .unwrap();
                }
                TryAcquireError::Closed => {
                    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                }
            },
        };

        let gzip_response = config.force_gzip
            || req
                .headers()
                .get(header::ACCEPT_ENCODING)
                .map(|acc| {
                    acc.to_str()
                        .unwrap_or("")
                        .split(',')
                        // Don't you dare allocate anything !
                        .any(|tok| tok.trim().eq_ignore_ascii_case("gzip"))
                })
                .unwrap_or(false);

        let current_depth = axum::extract::Query::<QueryParams>::try_from_uri(req.uri())
            .ok()
            .and_then(|q| q.page)
            .unwrap_or(1);

        let link_settings = LinkSettings::next(config, current_depth);

        poison::serve_poison(config, in_flight_permit, gzip_response, link_settings)
            .await
            .into_response()
    }))
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

    // This hits the poison source over the network; move to an integration test suite eventually.
    #[tokio::test]
    async fn happy_path_works() {
        let app = new_miasma_router(&TEST_CONFIG);

        let response = app
            .oneshot(Request::builder().uri("/foo").body(Body::empty()).unwrap())
            .await
            .unwrap();

        // could be 500 if the network is down or 200 if it works, but shouldn't be 429.
        assert_ne!(response.status(), StatusCode::TOO_MANY_REQUESTS);
    }

    #[tokio::test]
    async fn returns_429_when_max_in_flight_reached() {
        let app = new_miasma_router(&TEST_CONFIG);
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
