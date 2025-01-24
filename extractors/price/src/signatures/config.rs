use redis::Client;
use solana_client::rpc_client::GetConfirmedSignaturesForAddress2Config;
use solana_sdk::{commitment_config::CommitmentConfig, signature::Signature};
use std::str::FromStr;

use crate::{
    constants::IS_HISTORIC_MODE,
    crawl_status::queries::{get_newest_crawled_signature, get_oldest_crawled_signature},
};

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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SignaturesWindowError {
    HistoryComplete,
}

pub fn build_signatures_window_config(
    client: &Client,
    account_address: &str,
    limit: Option<usize>,
) -> Result<(Option<String>, Option<String>, usize), SignaturesWindowError> {
    let mut oldest_signature = None;
    let mut latest_signature = None;
    let limit = limit.unwrap_or(DEFAULT_SIGNATURES_LIMIT);

    if IS_HISTORIC_MODE {
        let oldest = get_oldest_crawled_signature(&client, &account_address);

        if let Ok(Some(signature)) = oldest {
            oldest_signature = Some(signature);
        }
    } else {
        let newest = get_newest_crawled_signature(&client, &account_address);

        if let Ok(Some(signature)) = newest {
            latest_signature = Some(signature);
        }
    }

    Ok((oldest_signature, latest_signature, limit))
}
