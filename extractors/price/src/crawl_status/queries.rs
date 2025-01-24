use crate::crawl_status::table::{
    REDIS_ACCOUNT_TRANSACTIONS_PREFIX, REDIS_TRANSACTION_STATUS_PREFIX,
};
use redis::{Client, RedisError};

pub fn get_newest_crawled_signature(
    client: &Client,
    account_address: &str,
) -> Result<Option<String>, RedisError> {
    let mut conn = client.get_connection()?;

    let account_key = format!("{}:{}", REDIS_ACCOUNT_TRANSACTIONS_PREFIX, account_address);

    let signatures: Vec<String> = redis::cmd("ZREVRANGE")
        .arg(&account_key)
        .arg(0)
        .arg(0)
        .query(&mut conn)?;

    if let Some(signature) = signatures.first() {
        let tx_key = format!("{}:{}", REDIS_TRANSACTION_STATUS_PREFIX, signature);
        let status: String = redis::cmd("HGET")
            .arg(&tx_key)
            .arg("status")
            .query(&mut conn)?;

        if status != "pending" {
            return Ok(Some(signature.clone()));
        }
    }

    Ok(None)
}

pub fn get_oldest_crawled_signature(
    client: &Client,
    account_address: &str,
) -> Result<Option<String>, RedisError> {
    let mut conn = client.get_connection()?;

    let account_key = format!("{}:{}", REDIS_ACCOUNT_TRANSACTIONS_PREFIX, account_address);

    let signatures: Vec<String> = redis::cmd("ZRANGE")
        .arg(&account_key)
        .arg(0)
        .arg(0)
        .query(&mut conn)?;

    if let Some(signature) = signatures.first() {
        let tx_key = format!("{}:{}", REDIS_TRANSACTION_STATUS_PREFIX, signature);
        let status: String = redis::cmd("HGET")
            .arg(&tx_key)
            .arg("status")
            .query(&mut conn)?;

        if status != "pending" {
            return Ok(Some(signature.clone()));
        }
    }

    Ok(None)
}
