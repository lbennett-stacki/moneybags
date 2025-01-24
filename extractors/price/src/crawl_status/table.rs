use clickhouse::{Client, Row};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

pub const CLICKHOUSE_CRAWL_STATUS_TABLE_NAME: &str = "crawl_status";

#[derive(Debug, Clone, Copy, PartialEq, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum CrawlStatus {
    Pending = 1,
    Failed = 2,
    Succeeded = 3,
}

#[derive(Debug, Clone, Row, Deserialize, Serialize)]
pub struct CrawlStatusRow {
    pub account_address: String,
    pub transaction_signature: String,
    pub slot: u64,
    pub status: CrawlStatus,
    pub is_first_account_signature: bool,
}

pub async fn create_crawl_status_table(client: &Client) -> Result<(), clickhouse::error::Error> {
    client
        .query(
            format!(
                "CREATE TABLE IF NOT EXISTS {} (
                account_address String,
                transaction_signature String,
                slot UInt64,
                is_first_account_signature Boolean,
                status Enum8('pending' = {}, 'failed' = {}, 'succeeded' = {}),
            )
            ENGINE = MergeTree()
            ORDER BY (slot, transaction_signature, account_address)
        ",
                CLICKHOUSE_CRAWL_STATUS_TABLE_NAME,
                CrawlStatus::Pending as u8,
                CrawlStatus::Failed as u8,
                CrawlStatus::Succeeded as u8,
            )
            .as_str(),
        )
        .execute()
        .await?;

    Ok(())
}
