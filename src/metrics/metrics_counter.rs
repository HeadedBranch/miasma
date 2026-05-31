use std::{collections::HashMap, mem};

use colored::Colorize;
use diesel::{prelude::*, upsert::excluded};

use crate::metrics::RESULTS_PER_PAGE;

use super::MetricsError;

use self::user_agents::dsl::*;

diesel::table! {
    user_agents (agent) {
        agent -> Text,
        count -> BigInt,
    }
}

pub struct Metrics {
    counts: HashMap<String, usize>,
    unflushed_count: u32,
    db_path: String,
}

impl Metrics {
    const MAX_UNFLUSHED_COUNT: u32 = 1_000;
    const MAX_USER_AGENT_CHAR_LENGTH: usize = 1024;

    pub fn new(db_path: String) -> Result<Self, MetricsError> {
        let mut conn = SqliteConnection::establish(&db_path)?;
        diesel::sql_query(
            "CREATE TABLE IF NOT EXISTS user_agents (
                agent TEXT PRIMARY KEY,
                count INTEGER NOT NULL
            )",
        )
        .execute(&mut conn)?;
        Ok(Self {
            db_path,
            counts: HashMap::new(),
            unflushed_count: 0,
        })
    }

    /// Increment the request count for the supplied user agent by one.
    pub fn count_request(&mut self, user_agent: &str) {
        // Truncate the user agent string to ensure we don't store massive values.
        // There's a very small chance that scrapers have big ole user agents and
        // might try to exploit the fact that we're storing them.
        let truncated_user_agent = user_agent
            .chars()
            .take(Metrics::MAX_USER_AGENT_CHAR_LENGTH)
            .collect();
        self.unflushed_count += 1;
        if let Some(c) = self.counts.get_mut(&truncated_user_agent) {
            *c += 1;
        } else {
            self.counts.insert(truncated_user_agent, 1);
        }
        if self.unflushed_count >= Metrics::MAX_UNFLUSHED_COUNT {
            self.unflushed_count = 0;
            self.flush_in_background();
        }
    }

    /// Flush metrics to the database in a non-blocking background task.
    pub fn flush_in_background(&mut self) {
        let flushing = mem::take(&mut self.counts);
        let db_path = self.db_path.clone();

        tokio::task::spawn_blocking(move || {
            flush_to_db(flushing, &db_path);
        });
    }

    /// Flush metrics to the database and block until completion.
    pub fn flush_blocking(&mut self) {
        let flushing = std::mem::take(&mut self.counts);

        flush_to_db(flushing, &self.db_path);
    }

    /// List a portion of entries in the metrics database by request count.
    pub fn list_useragents_by_count(
        &mut self,
        page: u32,
    ) -> Result<Vec<(String, i64)>, MetricsError> {
        let offset = page.saturating_sub(1) * RESULTS_PER_PAGE as u32;
        let mut conn = SqliteConnection::establish(&self.db_path)?;
        let entries = user_agents
            .order_by(count.desc())
            .limit(RESULTS_PER_PAGE as i64)
            .offset(offset as i64)
            .load::<(String, i64)>(&mut conn)?;
        Ok(entries)
    }
}

fn flush_to_db(counts: HashMap<String, usize>, db_path: &str) {
    let mut conn = match SqliteConnection::establish(db_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}: {e}", "Failed to connect to metrics database".red());
            return;
        }
    };
    #[allow(unused_must_use)]
    diesel::sql_query("PRAGMA busy_timeout = 5000").execute(&mut conn);

    let rows = counts
        .into_iter()
        .map(|(ua, c)| (agent.eq(ua), count.eq(c as i64)))
        .collect::<Vec<_>>();

    if let Err(e) = diesel::insert_into(user_agents)
        .values(rows)
        .on_conflict(agent)
        .do_update()
        .set(count.eq(count + excluded(count)))
        .execute(&mut conn)
    {
        eprintln!(
            "{}: {e}",
            "Failed to write User-Agent counts to database".red()
        );
    }
}

// Ensure metrics are flushed when going out of scope.
impl Drop for Metrics {
    fn drop(&mut self) {
        self.flush_blocking();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn counts_persisted_on_flush() {
        let db_file = NamedTempFile::new()
            .unwrap()
            .path()
            .to_str()
            .unwrap()
            .to_owned();

        let mut conn = SqliteConnection::establish(&db_file).expect("failed to connect to test db");
        diesel::sql_query(
            "CREATE TABLE user_agents (
                agent TEXT PRIMARY KEY,
                count INTEGER NOT NULL
            )",
        )
        .execute(&mut conn)
        .expect("failed to create test table");

        let expected = [
            ("miasma/0.1".to_owned(), 5),
            ("claudebot".to_owned(), 10),
            ("safari".to_owned(), 15),
        ];

        flush_to_db(HashMap::from(expected.clone()), &db_file);

        let mut conn =
            SqliteConnection::establish(&db_file).expect("failed to connect to database");

        let rows = user_agents
            .load::<(String, i64)>(&mut conn)
            .expect("failed to query test db");

        assert_eq!(rows.len(), expected.len());
        for (expected_ua, expected_count) in expected {
            let (actual_ua, actual_count) = rows
                .iter()
                .find(|(ua, _)| ua.as_str() == expected_ua)
                .expect("expected row not found in test db");
            assert_eq!(actual_ua, &expected_ua);
            assert_eq!(*actual_count, expected_count as i64);
        }
    }
}
