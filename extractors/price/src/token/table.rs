use clickhouse::Client;

pub const CLICKHOUSE_TOKENS_TABLE_NAME: &str = "tokens";

#[derive(Debug, Clone)]
pub struct TokenRow {
    pub mint_address: String,
    pub bonding_curve_address: String,
    pub decimals: u8,
}

pub async fn create_tokens_table(client: &Client) -> Result<(), clickhouse::error::Error> {
    client
        .query(
            format!(
                "CREATE TABLE IF NOT EXISTS {} (
                mint_address String,
                bonding_curve_address String,
                decimals UInt8
            )
            ENGINE = MergeTree()
            ORDER BY (mint_address)
        ",
                CLICKHOUSE_TOKENS_TABLE_NAME,
            )
            .as_str(),
        )
        .execute()
        .await?;

    Ok(())
}
