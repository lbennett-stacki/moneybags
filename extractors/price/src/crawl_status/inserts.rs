use crate::crawl_status::table::{CrawlStatus, CrawlStatusRow, REDIS_TRANSACTION_STATUS_PREFIX};
use redis::{Client, RedisError};

pub fn add_account_signature(
    client: &Client,
    crawl_status: &CrawlStatusRow,
    batch_size: usize,
) -> Result<(), RedisError> {
    let mut conn = client.get_connection()?;

    let composite_score = (crawl_status.slot as f64)
        + (crawl_status.relative_transaction_index as f64 / batch_size as f64);

    let _: () = redis::cmd("ZADD")
        .arg(crawl_status.account_transactions_key())
        .arg(composite_score)
        .arg(&crawl_status.transaction_signature)
        .query(&mut conn)?;

    Ok(())
}

pub fn insert_crawl_status(
    client: &Client,
    crawl_status: &CrawlStatusRow,
    batch_size: usize,
) -> Result<(), RedisError> {
    let mut conn = client.get_connection()?;

    let status_str = match crawl_status.status {
        CrawlStatus::Pending => "pending",
        CrawlStatus::Failed => "failed",
        CrawlStatus::Succeeded => "succeeded",
    };

    add_account_signature(client, crawl_status, batch_size)?;

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
    error: &String,
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

pub fn mark_first_account_signature(
    client: &Client,
    transaction_signature: &str,
) -> Result<(), RedisError> {
    let mut conn = client.get_connection()?;
    let tx_key = format!(
        "{}:{}",
        REDIS_TRANSACTION_STATUS_PREFIX, transaction_signature
    );

    println!(" tx_key: {}", tx_key);

    let _: () = redis::cmd("HSET")
        .arg(&tx_key)
        .arg("is_first_account_signature")
        .arg("true")
        .query(&mut conn)?;

    Ok(())
}
