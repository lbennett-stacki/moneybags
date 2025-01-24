use crate::crawl_status::table::{CrawlStatus, CrawlStatusRow, REDIS_TRANSACTION_STATUS_PREFIX};
use redis::{Client, RedisError};

pub fn insert_crawl_status(
    client: &Client,
    crawl_status: &CrawlStatusRow,
) -> Result<(), RedisError> {
    let mut conn = client.get_connection()?;

    let status_str = match crawl_status.status {
        CrawlStatus::Pending => "pending",
        CrawlStatus::Failed => "failed",
        CrawlStatus::Succeeded => "succeeded",
    };

    let _: () = redis::cmd("ZADD")
        .arg(crawl_status.account_transactions_key())
        .arg(crawl_status.slot)
        .arg(&crawl_status.transaction_signature)
        .query(&mut conn)?;

    let mut cmd = redis::cmd("HSET");
    cmd.arg(crawl_status.transaction_status_key())
        .arg("account_address")
        .arg(&crawl_status.account_address)
        .arg("transaction_signature")
        .arg(&crawl_status.transaction_signature)
        .arg("slot")
        .arg(crawl_status.slot.to_string())
        .arg("status")
        .arg(status_str)
        .arg("is_first_account_signature")
        .arg(crawl_status.is_first_account_signature.to_string());

    if let Some(error) = &crawl_status.error {
        cmd.arg("error").arg(error);
    }

    let _: () = cmd.query(&mut conn)?;

    Ok(())
}

pub fn mark_crawl_success(client: &Client, transaction_signature: &str) -> Result<(), RedisError> {
    let mut conn = client.get_connection()?;
    let tx_key = format!(
        "{}:{}",
        REDIS_TRANSACTION_STATUS_PREFIX, transaction_signature
    );

    let _: () = redis::cmd("HDEL")
        .arg(&tx_key)
        .arg("error")
        .query(&mut conn)?;

    let _: () = redis::cmd("HSET")
        .arg(&tx_key)
        .arg("status")
        .arg("succeeded")
        .query(&mut conn)?;

    Ok(())
}

pub fn mark_crawl_failed(
    client: &Client,
    transaction_signature: &str,
    error: &str,
) -> Result<(), RedisError> {
    let mut conn = client.get_connection()?;
    let tx_key = format!(
        "{}:{}",
        REDIS_TRANSACTION_STATUS_PREFIX, transaction_signature
    );

    let _: () = redis::cmd("HSET")
        .arg(&tx_key)
        .arg("status")
        .arg("failed")
        .arg("error")
        .arg(error)
        .query(&mut conn)?;

    Ok(())
}
