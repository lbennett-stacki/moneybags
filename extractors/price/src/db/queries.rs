use crate::crawl_status::table::{CrawlStatusRow, CLICKHOUSE_CRAWL_STATUS_TABLE_NAME};
use clickhouse::Client;

pub async fn get_latest_account_signature(
    client: &Client,
    account_address: &str,
) -> Result<Option<CrawlStatusRow>, clickhouse::error::Error> {
    let row = client
        .query(
            format!(
                "
        SELECT account_address, transaction_signature, slot, status, is_first_account_signature
        FROM {} 
        WHERE account_address = '{}'
        ORDER BY slot DESC
        LIMIT 1
    ",
                CLICKHOUSE_CRAWL_STATUS_TABLE_NAME, account_address,
            )
            .as_str(),
        )
        .fetch::<CrawlStatusRow>()?
        .next()
        .await;

    row
}

pub async fn get_oldest_account_signature(
    client: &Client,
    account_address: &str,
) -> Result<Option<CrawlStatusRow>, clickhouse::error::Error> {
    let row = client
        .query(
            format!(
                "
        SELECT account_address, transaction_signature, slot, status, is_first_account_signature
        FROM {} 
        WHERE account_address = '{}'
        ORDER BY slot ASC
        LIMIT 1
    ",
                CLICKHOUSE_CRAWL_STATUS_TABLE_NAME, account_address,
            )
            .as_str(),
        )
        .fetch::<CrawlStatusRow>()?
        .next()
        .await;

    row
}
