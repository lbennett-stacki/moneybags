use clickhouse::Client;
use time::OffsetDateTime;

pub const CLICKHOUSE_TRADES_TABLE_NAME: &str = "trades";

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TradeDirection {
    Buy = 1,
    Sell = 2,
}

#[derive(Debug, Clone)]
pub struct TradeRow {
    pub coin_token_address: String,
    pub price_coin_token_address: String,
    pub transaction_signature: String,
    pub slot: u64,
    pub instruction_index: u64,
    pub block_time: OffsetDateTime,
    pub coin_token_amount: u64,
    pub price_coin_token_amount: u64,
    pub direction: TradeDirection,
}

pub async fn create_trades_table(client: &Client) -> Result<(), clickhouse::error::Error> {
    client
        .query(
            format!(
                "
        CREATE TABLE IF NOT EXISTS {} (
            coin_token_address String,
            price_coin_token_address String,
            transaction_signature String,
            slot UInt64,
            instruction_index UInt64,
            block_time DateTime,
            coin_token_amount UInt64,
            price_coin_token_amount UInt64,
            direction Enum8('buy' = {}, 'sell' = {}),
        )
        ENGINE = MergeTree()
        ORDER BY (slot, transaction_signature, coin_token_address, instruction_index)
    ",
                CLICKHOUSE_TRADES_TABLE_NAME,
                TradeDirection::Buy as u8,
                TradeDirection::Sell as u8,
            )
            .as_str(),
        )
        .execute()
        .await?;

    Ok(())
}
