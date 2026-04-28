use std::sync::Arc;

use axum::{
    Router,
    body::Body,
    extract::State,
    http::Request,
    response::{IntoResponse, Response},
    routing::get,
};
use reqwest::{StatusCode, header};
use serde::Deserialize;
use tokio::sync::{Semaphore, TryAcquireError};

use crate::MiasmaConfig;
use crate::logger::Logger;
use crate::poison::{self, LinkSettings};

#[derive(Deserialize)]
pub struct QueryParams {
    /// We use 'page' instead of depth to look more convincing to scrapers
    page: Option<u32>,
}
impl QueryParams {
    pub const CURRENT_DEPTH_QUERY_PARAM: &str = "page";
}

#[derive(Clone)]
struct AppState<'a> {
    counter: &'a Logger,
    config: &'static MiasmaConfig,
    in_flight_sem: Arc<Semaphore>,
}

/// Build a new `axum::Router` for Miasma's routes.
pub fn new_miasma_router(config: &'static MiasmaConfig, counter: &'static Logger) -> Router {
    Router::new().fallback(get(handler)).with_state(AppState {
        config,
        in_flight_sem: Arc::new(Semaphore::new(config.max_in_flight as usize)),
        counter,
    })
}

async fn handler(State(state): State<AppState<'_>>, req: Request<Body>) -> impl IntoResponse {
    let in_flight_permit = match state.in_flight_sem.try_acquire_owned() {
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

    let headers = req.headers();

    if req.uri() == "/stats" {
        if headers
            .get(header::AUTHORIZATION)
            .is_some_and(|h| h == "Basic dXNlcm5hbWU6cGFzc3dvcmQ=")
        {
            state.counter.serve_stats().await.into_response()
        } else {
            Response::builder()
                .header(header::WWW_AUTHENTICATE, "Basic")
                .status(401)
                .body(Body::new(String::new()))
                .unwrap()
        }
    } else if req.uri() == "/favicon.ico" {
        // Returns the 🌀 emoji
        Response::builder()
            .status(200)
            .header(header::CONTENT_TYPE, "image/svg+xml")
            .body(Body::new(String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" xml:space=\"preserve\" viewBox=\"0 0 128 128\"><path d=\"M65.79 57.85c-2.04 2.3-1.21 5.16-.07 6.57 2.13 2.64 5.29 2.79 6.97 6.12 1.48 2.93.59 8.25-5.96 8.69s-13.05-4.88-14.43-11.2c-1.71-7.85.63-16.61 9.88-21.1s21.4-2.18 28.11 5.01c6.94 7.42 11.86 17.04 8.54 30.96s-17.9 25.82-40.73 23.45c-18.8-1.95-36.11-18.96-35.28-44.64.81-25.02 21.6-40.36 41.13-43.45 20.19-3.19 34.42 5.47 42.73 12.08 5.17 4.12 9.62 8.72 10.9 10.13 1.78 1.96 4.48 2.06 4.85-1.08.49-4.15-2.09-9.81-8.11-16.91-6.02-7.09-23.48-20.88-52.14-18.2C18.08 8.39 2.89 47.62 5.9 72.01s20.3 51.23 60.24 52.07c24.42.51 43.29-14.96 49.16-35.43 3.25-11.33 2.23-25.66-4.64-36.82-9.23-14.97-19.76-21.05-34.16-22.5-11.53-1.16-22.47 2.17-29.67 8.64-14.7 13.2-15.15 37.01.17 48.95 8.09 6.31 17.73 7.47 25.14 5.18 5.09-1.57 9.53-4.28 12.14-9.34 3.1-6.02 2.8-11.93.66-16.56-1.77-3.84-5.52-7.07-8.79-8.72-4.93-2.47-8.44-1.79-10.36.37\" style=\"fill:#1f87fd\"/></svg>"))).unwrap()
    } else {
        let gzip_response = state.config.force_gzip
            || headers.get(header::ACCEPT_ENCODING).is_some_and(|acc| {
                acc.to_str()
                    .unwrap_or("")
                    .split(',')
                    // Don't you dare allocate anything !
                    .any(|tok| tok.trim().eq_ignore_ascii_case("gzip"))
            });

        let current_depth = axum::extract::Query::<QueryParams>::try_from_uri(req.uri())
            .ok()
            .and_then(|q| q.page)
            .unwrap_or(1);

        let user_agent = if headers.contains_key(header::USER_AGENT) {
            headers.get(header::USER_AGENT).unwrap().to_str().unwrap()
        } else {
            "No-Agent"
        }
        .to_string();

        state.counter.add_record(user_agent);

        let link_settings = LinkSettings::next(state.config, current_depth);

        poison::serve_poison(state.config, in_flight_permit, gzip_response, link_settings)
            .await
            .into_response()
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use axum::{
//         body::Body,
//         http::{Request, StatusCode, header::RETRY_AFTER},
//     };
//     use std::sync::LazyLock;
//     use tower::ServiceExt;
//
//     static TEST_CONFIG: LazyLock<MiasmaConfig> = LazyLock::new(|| MiasmaConfig {
//         max_in_flight: 1,
//         ..Default::default()
//     });
//
//     // This hits the poison source over the network; move to an integration test suite eventually.
//     #[tokio::test]
//     async fn happy_path_works() {
//         let app = new_miasma_router(&TEST_CONFIG, Logger::new());
//
//         let response = app
//             .oneshot(Request::builder().uri("/foo").body(Body::empty()).unwrap())
//             .await
//             .unwrap();
//
//         // could be 500 if the network is down or 200 if it works, but shouldn't be 429.
//         assert_ne!(response.status(), StatusCode::TOO_MANY_REQUESTS);
//     }
//
//     #[tokio::test]
//     async fn returns_429_when_max_in_flight_reached() {
//         let app = new_miasma_router(&TEST_CONFIG, Logger::new());
//         let req1 = Request::builder().uri("/foo").body(Body::empty()).unwrap();
//         let req2 = Request::builder().uri("/foo").body(Body::empty()).unwrap();
//
//         let (res1, res2) = tokio::join!(app.clone().oneshot(req1), app.oneshot(req2));
//
//         let res1 = res1.unwrap();
//         let res2 = res2.unwrap();
//
//         let limited = if res1.status() == StatusCode::TOO_MANY_REQUESTS {
//             res1
//         } else if res2.status() == StatusCode::TOO_MANY_REQUESTS {
//             res2
//         } else {
//             panic!(
//                 "expected one 429, got {} and {}",
//                 res1.status(),
//                 res2.status()
//             );
//         };
//
//         assert_eq!(limited.status(), StatusCode::TOO_MANY_REQUESTS);
//         assert_eq!(
//             limited
//                 .headers()
//                 .get(RETRY_AFTER)
//                 .and_then(|v| v.to_str().ok()),
//             Some("5")
//         );
//     }
// }
