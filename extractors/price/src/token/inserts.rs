use crate::token::table::{TokenRow, CLICKHOUSE_TOKENS_TABLE_NAME};
use clickhouse::Client;

pub async fn insert_token_decimals(
    client: &Client,
    token: &TokenRow,
) -> Result<(), clickhouse::error::Error> {
    client
        .query(
            format!(
                "INSERT INTO {} (
                    mint_address,
                    bonding_curve_address,
                    decimals,
                ) VALUES ('{}', '{}', {})",
                CLICKHOUSE_TOKENS_TABLE_NAME,
                token.mint_address,
                token.bonding_curve_address,
                token.decimals,
            )
            .as_str(),
        )
        .execute()
        .await?;
    Ok(())
}
