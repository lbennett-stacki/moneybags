use clickhouse::Client;
use solana_sdk::pubkey::Pubkey;

use crate::pump_fun::tokens::MintAddress;

pub const CLICKHOUSE_RAYDIUM_POOLS_TABLE_NAME: &str = "raydium_pools";

pub type PoolAddress = Pubkey;

#[derive(Debug, Clone)]
pub struct RaydiumPoolRow {
    pub pool_address: PoolAddress,
    pub mint_address: MintAddress,
}

pub async fn create_raydium_pools_table(client: &Client) -> Result<(), clickhouse::error::Error> {
    client
        .query(
            format!(
                "
        CREATE TABLE IF NOT EXISTS {} (
            pool_address String,
            mint_address String,
        )
        ENGINE = MergeTree()
        ORDER BY (pool_address, mint_address)
    ",
                CLICKHOUSE_RAYDIUM_POOLS_TABLE_NAME,
            )
            .as_str(),
        )
        .execute()
        .await?;

    Ok(())
}
