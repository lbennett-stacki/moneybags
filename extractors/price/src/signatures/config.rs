use clickhouse::Client;
use solana_client::rpc_client::GetConfirmedSignaturesForAddress2Config;
use solana_sdk::{commitment_config::CommitmentConfig, signature::Signature};
use std::str::FromStr;

use crate::{
    constants::IS_HISTORIC_MODE,
    db::queries::{get_latest_account_signature, get_oldest_account_signature},
    utils::blocking::blocking_call,
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
    db_client: &Client,
    account_address: &str,
    limit: Option<usize>,
) -> Result<(Option<String>, Option<String>, usize), SignaturesWindowError> {
    let mut oldest_signature = None;
    let mut latest_signature = None;
    let limit = limit.unwrap_or(DEFAULT_SIGNATURES_LIMIT);

    if IS_HISTORIC_MODE {
        let oldest = blocking_call(async move {
            get_oldest_account_signature(&db_client, &account_address).await
        });

        if let Ok(Some(row)) = oldest {
            if row.is_first_account_signature {
                return Err(SignaturesWindowError::HistoryComplete);
            }
            oldest_signature = Some(row.transaction_signature);
        }
    } else {
        let latest = blocking_call(async move {
            get_latest_account_signature(&db_client, &account_address).await
        });

        if let Ok(Some(row)) = latest {
            latest_signature = Some(row.transaction_signature);
        }
    }

    Ok((oldest_signature, latest_signature, limit))
}
