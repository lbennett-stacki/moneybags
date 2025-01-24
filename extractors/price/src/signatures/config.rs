use redis::Client;
use solana_client::rpc_client::GetConfirmedSignaturesForAddress2Config;
use solana_sdk::{commitment_config::CommitmentConfig, signature::Signature};
use std::str::FromStr;

use crate::crawl_status::queries::{get_oldest_seen_signature, CrawlStatusQueryError};

pub const DEFAULT_SIGNATURES_LIMIT: usize = 1000;

pub fn build_signatures_config(
    before: Option<String>,
    until: Option<String>,
    limit: Option<usize>,
) -> GetConfirmedSignaturesForAddress2Config {
    GetConfirmedSignaturesForAddress2Config {
        commitment: Some(CommitmentConfig::confirmed()),
        before: before.map(|before| Signature::from_str(&before).unwrap()),
        until: until.map(|until| Signature::from_str(&until).unwrap()),
        limit: Some(limit.unwrap_or(DEFAULT_SIGNATURES_LIMIT)),
    }
}

pub fn build_signatures_window_config(
    client: &Client,
    account_address: &str,
    limit: Option<usize>,
) -> Result<(Option<String>, usize), CrawlStatusQueryError> {
    let limit = limit.unwrap_or(DEFAULT_SIGNATURES_LIMIT);

    let oldest_signature = get_oldest_seen_signature(&client, &account_address)?;

    Ok((oldest_signature, limit))
}
