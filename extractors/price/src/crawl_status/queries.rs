use crate::crawl_status::table::{
    REDIS_ACCOUNT_TRANSACTIONS_PREFIX, REDIS_TRANSACTION_STATUS_PREFIX,
};
use redis::Client;

use super::errors::CrawlStatusQueryError;

fn has_signature_status(
    client: &Client,
    signature: &str,
    statuses: Vec<&str>,
) -> Result<bool, CrawlStatusQueryError> {
    let mut conn = client
        .get_connection()
        .map_err(|e| CrawlStatusQueryError::Redis(e))?;

    let key = format!("{}:{}", REDIS_TRANSACTION_STATUS_PREFIX, signature);
    let status: String = redis::cmd("HGET")
        .arg(&key)
        .arg("status")
        .query(&mut conn)
        .map_err(|e| CrawlStatusQueryError::Redis(e))?;

    Ok(statuses.contains(&status.as_str()))
}

fn is_first_account_signature(
    client: &Client,
    signature: &str,
) -> Result<bool, CrawlStatusQueryError> {
    let mut conn = client
        .get_connection()
        .map_err(|e| CrawlStatusQueryError::Redis(e))?;

    let key = format!("{}:{}", REDIS_TRANSACTION_STATUS_PREFIX, signature);
    let is_first_account_signature: String = redis::cmd("HGET")
        .arg(&key)
        .arg("is_first_account_signature")
        .query(&mut conn)
        .map_err(|e| CrawlStatusQueryError::Redis(e))?;
    let is_first_account_signature = is_first_account_signature == "true";

    Ok(is_first_account_signature)
}

pub fn has_crawled_signature(
    client: &Client,
    signature: &str,
) -> Result<bool, CrawlStatusQueryError> {
    has_signature_status(client, signature, vec!["succeeded", "failed"])
}

pub fn get_oldest_seen_signature(
    client: &Client,
    account_address: &str,
) -> Result<Option<String>, CrawlStatusQueryError> {
    let mut conn = client
        .get_connection()
        .map_err(|e| CrawlStatusQueryError::Redis(e))?;

    let account_key = format!("{}:{}", REDIS_ACCOUNT_TRANSACTIONS_PREFIX, account_address);

    let signatures: Vec<String> = redis::cmd("ZRANGE")
        .arg(&account_key)
        .arg(0)
        .arg(0)
        .query::<Vec<String>>(&mut conn)
        .map_err(|e| CrawlStatusQueryError::Redis(e))?;

    if signatures.is_empty() {
        return Ok(None);
    }

    if is_first_account_signature(client, signatures.first().unwrap())? {
        println!("History complete for {} !!!!!", account_address);
        return Err(CrawlStatusQueryError::HistoryComplete);
    }

    Ok(signatures.first().cloned())
}
