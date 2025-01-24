use crate::token::table::CLICKHOUSE_TOKENS_TABLE_NAME;
use clickhouse::Client;

pub async fn has_token(
    client: &Client,
    mint_address: &str,
) -> Result<bool, clickhouse::error::Error> {
    let result = client
        .query(
            format!(
                "SELECT COUNT(*) FROM {} WHERE mint_address = '{}'",
                CLICKHOUSE_TOKENS_TABLE_NAME, mint_address,
            )
            .as_str(),
        )
        .fetch_one::<u64>()
        .await?;

    Ok(result > 0)
}
