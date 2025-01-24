use crate::raydium::table::{RaydiumPoolRow, CLICKHOUSE_RAYDIUM_POOLS_TABLE_NAME};
use clickhouse::Client;

pub async fn insert_raydium_pool(
    client: &Client,
    raydium_pool: &RaydiumPoolRow,
) -> Result<(), clickhouse::error::Error> {
    client
        .query(
            format!(
                "
            INSERT INTO {} (
                pool_address,
                mint_address,
            ) VALUES (
               '{}',
               '{}',
            )
            ",
                CLICKHOUSE_RAYDIUM_POOLS_TABLE_NAME,
                raydium_pool.pool_address,
                raydium_pool.mint_address,
            )
            .as_str(),
        )
        .execute()
        .await
        .unwrap();

    Ok(())
}
