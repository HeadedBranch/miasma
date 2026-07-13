use async_stream::try_stream;
use axum::{
    body::Body,
    extract::{Query, State},
    http::{HeaderMap, Response},
    response::IntoResponse,
};
use bytes::Bytes;
use futures::Stream;
use reqwest::{StatusCode, header};
use serde::Deserialize;

use crate::metrics::{MetricsError, MetricsState, RESULTS_PER_PAGE};

#[derive(Deserialize)]
pub(crate) struct QueryParams {
    page: Option<u32>,
}

pub async fn metrics_handler(
    State(state): State<MetricsState>,
    Query(QueryParams { page }): Query<QueryParams>,
    headers: HeaderMap,
) -> impl IntoResponse {
    if headers
        .get(header::AUTHORIZATION)
        .is_none_or(|creds| creds != state.auth_value.as_ref())
    {
        return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .header(header::WWW_AUTHENTICATE, "Basic")
            .body(Body::empty())
            .unwrap();
    }

    let mut counter = state.counter.lock().await;
    counter.flush_blocking();

    let page_number = page.map_or(1, |p| if p == 0 { 1 } else { p });
    let entries = match counter.list_useragents_by_count(page_number) {
        Ok(v) => v,
        Err(e) => {
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                // MetricsError has an appropriate debug message for this case
                .body(Body::from(format!("{e:?}")))
                .unwrap();
        }
    };

    let stream = stream_metrics_page(page_number, entries);

    Response::builder()
        .header(header::CONTENT_TYPE, "text/html")
        .body(Body::from_stream(stream))
        .unwrap()
}

fn stream_metrics_page(
    page_number: u32,
    entries: Vec<(String, (i64, i64))>,
) -> impl Stream<Item = Result<Bytes, MetricsError>> {
    try_stream! {
        // 🐦‍⬛ is this a jsx? 🦋
        yield Bytes::from(fhtml::concat!{
            <!DOCTYPE html>
            <html lang="en">
            <head>
                <meta charset="UTF-8">
                <link rel="icon" href={include_str!("miasma-swirl.txt")}>
                <title>"Miasma Metrics"</title>
                <style>{include_str!("metrics.css")}</style>
            </head>
            <body>
                <h1>"🌀 Miasma"</h1>
                <h2>"Request Counts by Scraper User-Agent"</h2>
                <table>
                    <thead>
                        <tr>
                            <th></th>
                            <th>"User-Agent"</th>
                            <th>"Request Count"</th>
                            <th>"Bytes Sent"</th>
                        </tr>
                    </thead>
                    <tbody>
        });

        for (ind, (user_agent, (count, bytes))) in entries.iter().enumerate() {
            yield Bytes::from(fhtml::format!{
                        <tr>
                            <td>{ind+1 + (page_number.saturating_sub(1) * RESULTS_PER_PAGE) as usize}</td>
                            // Escape the user agent string in case
                            // scrapers try to send us JavaScript.
                            // Very unlikely but a non-zero chance...
                            <td class="user-agent">{fhtml::escape(user_agent)}</td>
                            <td class="request-count">{count}</td>
                            <td class="request-bytes">{bytes}</td>
                        </tr>
            });
        }

        let prev_page = page_number.saturating_sub(1);
        let next_page = page_number + 1;
        yield Bytes::from(fhtml::format!{
                    </tbody>
                </table>
                <div class="pagination">
                    <a
                        href={format!("?page={prev_page}")}
                        class={format!("link{}", if prev_page == 0 {"-disabled"} else {""})}
                    >
                        "🡐 Previous"
                    </a>
                    <a
                        href={format!("?page={next_page}")}
                        class={format!("link{}", if entries.len() < RESULTS_PER_PAGE as usize {"-disabled"} else {""})}
                    >
                        "Next 🡒"
                    </a>
                </div>
            </body>
            </html>
        });
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use futures::StreamExt;

    #[tokio::test]
    async fn page_stream_produces_valid_html() {
        let entries = vec![("miasma/0.1".to_owned(), (1, 1024))];
        let page = stream_metrics_page(1, entries)
            .map(|chunk| String::from_utf8(chunk.unwrap().to_vec()).unwrap())
            .collect::<String>()
            .await;
        let errors = scraper::Html::parse_document(&page).errors;
        assert!(errors.is_empty());
    }
}
