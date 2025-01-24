use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::pump_fun::program::signatures::TransactionSignature;

pub const REDIS_ACCOUNT_TRANSACTIONS_PREFIX: &str = "account";
pub const REDIS_TRANSACTION_STATUS_PREFIX: &str = "tx";

#[derive(Debug, Clone, Copy, PartialEq, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum CrawlStatus {
    Pending = 1,
    Failed = 2,
    Succeeded = 3,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CrawlStatusRow {
    pub account_address: String,
    pub transaction_signature: String,
    pub slot: u64,
    pub relative_transaction_index: u64,
    pub status: CrawlStatus,
    pub is_first_account_signature: bool,
    pub error: Option<String>,
}

#[derive(Debug)]
pub enum CrawlStatusOperation {
    Create(CrawlStatusRow),
    MarkAsSucceeded(TransactionSignature),
    MarkAsFailed(TransactionSignature, String),
    MarkAsFirstAccountSignature(TransactionSignature),
}

impl CrawlStatusRow {
    pub fn account_transactions_key(&self) -> String {
        format!(
            "{}:{}",
            REDIS_ACCOUNT_TRANSACTIONS_PREFIX, self.account_address
        )
    }

    pub fn transaction_status_key(&self) -> String {
        format!(
            "{}:{}",
            REDIS_TRANSACTION_STATUS_PREFIX, self.transaction_signature
        )
    }
}
