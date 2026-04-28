use std::sync::{Arc, Mutex};

use self::useragents::dsl::*;
use dashmap::DashMap;
use diesel::{
    prelude::*,
    sql_types::{Integer, Text},
};

use crate::MiasmaStream;
use async_stream::try_stream;
use axum::body::Body;
use axum::response::{IntoResponse, Response};
use bytes::Bytes;
use reqwest::{StatusCode, header};

diesel::table! {
    useragents (agent) {
        agent -> Text,
        count -> Integer,
    }
}

pub struct Logger {
    db_conn: Arc<Mutex<SqliteConnection>>,
    memory_map: DashMap<String, i32>,
    counter: Arc<Mutex<u32>>,
}

impl Logger {
    pub fn new() -> Self {
        let mut db_conn = SqliteConnection::establish("test.db").unwrap();
        diesel::sql_query(
            "CREATE TABLE IF NOT EXISTS useragents (
                agent TEXT PRIMARY KEY,
                count INTEGER
            )",
        )
        .execute(&mut db_conn)
        .expect("failed to create useragents table");
        let memory_map = DashMap::<String, i32>::new();
        for (k, v) in useragents.load::<(String, i32)>(&mut db_conn).unwrap() {
            memory_map.insert(k, v);
        }
        Logger {
            db_conn: Arc::new(Mutex::new(db_conn)),
            memory_map,
            counter: Arc::new(Mutex::new(0)),
        }
    }
    pub fn add_record(&self, user_agent: String) {
        *self.counter.lock().unwrap() += 1;
        if let Some(mut c) = self.memory_map.get_mut(&user_agent) {
            *c += 1;
        } else {
            self.memory_map.insert(user_agent.clone(), 1);
        }
        // Writes to the database every 100 records
        if *self.counter.lock().unwrap() > 100 {
            *self.counter.lock().unwrap() = 0;
            self.flush_to_db();
        }
    }
    pub fn flush_to_db(&self) {
        for (user_agent, req_count) in self.memory_map.clone() {
            if let Err(e) = diesel::sql_query(
                "INSERT INTO useragents (agent, count)
                VALUES (?, ?)
                ON CONFLICT(agent)
                DO UPDATE SET count = excluded.count",
            )
            .bind::<Text, _>(user_agent)
            .bind::<Integer, _>(req_count)
            .execute(&mut *self.db_conn.lock().unwrap())
            {
                eprintln!("error writing to db: {e}");
            }
        }
    }
    pub async fn serve_stats(&self) -> impl IntoResponse {
        let body = build_html(self.memory_map.clone()).await;
        let body = Body::from_stream(body);
        let builder = Response::builder().header(header::CONTENT_TYPE, "text/html");
        builder.body(body).unwrap_or_else(|e| {
            eprintln!("Failed to build poison route response: {e}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        })
    }
}
async fn build_html(data: DashMap<String, i32>) -> impl MiasmaStream {
    try_stream! {
        yield Bytes::from_static(b"<table><thead><tr><th>User-Agent</th><th>Request Count</th></tr></thead><tbody>");
        for (user_agent, req_count) in data {
            yield Bytes::from(format!("<tr><th>{user_agent}</th><th>{req_count}</th></tr>"));
        }
        yield Bytes::from_static(b"</tbody></table>");
    }
}
