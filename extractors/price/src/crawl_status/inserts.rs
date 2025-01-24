use crate::crawl_status::table::{CrawlStatus, CrawlStatusRow, CLICKHOUSE_CRAWL_STATUS_TABLE_NAME};
use clickhouse::Client;

pub async fn insert_crawl_status(
    client: &Client,
    crawl_status: &CrawlStatusRow,
) -> Result<(), clickhouse::error::Error> {
    client
        .query(
            format!(
                "INSERT INTO {} (
                    account_address,
                    transaction_signature,
                    slot,
                    status,
                    is_first_account_signature,
                ) VALUES (
                    '{}',
                    '{}',
                     {},
                     '{}',
                     {}
                )",
                CLICKHOUSE_CRAWL_STATUS_TABLE_NAME,
                crawl_status.account_address,
                crawl_status.transaction_signature,
                crawl_status.slot,
                match crawl_status.status {
                    CrawlStatus::Pending => "pending",
                    CrawlStatus::Failed => "failed",
                    CrawlStatus::Succeeded => "succeeded",
                },
                crawl_status.is_first_account_signature,
            )
            .as_str(),
        )
        .execute()
        .await?;
    Ok(())
}
