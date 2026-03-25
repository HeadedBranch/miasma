use colored::Colorize;
use serde::Deserialize;
use std::time::Duration;

use crate::USER_AGENT;

#[derive(Deserialize)]
struct CrateInfo {
    max_stable_version: String,
}

#[derive(Deserialize)]
struct CratesIOResponse {
    #[serde(rename = "crate")]
    crate_info: CrateInfo,
}

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub async fn check_for_new_version() {
    let _res: anyhow::Result<()> = async {
        let resp = reqwest::Client::builder()
            .user_agent(USER_AGENT)
            .timeout(Duration::from_secs(5))
            .build()?
            .get("https://crates.io/api/v1/crates/miasma")
            .send()
            .await?
            .json::<CratesIOResponse>()
            .await?;

        let latest = resp.crate_info.max_stable_version;

        if VERSION != latest {
            eprintln!("\n------- New Version Available -------");
            eprintln!(
                "Installed ({}) -> Latest ({})",
                VERSION.red(),
                latest.green()
            );
            eprintln!("To update, run: {}", "cargo install miasma".blue());
            eprintln!("-------------------------------------\n");
        }

        Ok(())
    }
    .await;

    #[cfg(debug_assertions)]
    if let Err(e) = _res {
        eprintln!("Failed to check for latest version: {e}");
    }
}
