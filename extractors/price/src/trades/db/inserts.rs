use crate::trades::db::table::{TradeDirection, TradeRow, CLICKHOUSE_TRADES_TABLE_NAME};
use clickhouse::Client;

pub async fn insert_trade(
    client: &Client,
    trade: &TradeRow,
) -> Result<(), clickhouse::error::Error> {
    client
        .query(
            format!(
                "
            INSERT INTO {} (
                coin_token_address,
                price_coin_token_address,
                transaction_signature,
                slot,
                instruction_index,
                block_time,
                coin_token_amount,
                price_coin_token_amount,
                direction
            ) VALUES (
               '{}',
               '{}',
               '{}',
                {},
                {},
                {},
                {},
                {},
               '{}',
            )
            ",
                CLICKHOUSE_TRADES_TABLE_NAME,
                trade.coin_token_address,
                trade.price_coin_token_address,
                trade.transaction_signature,
                trade.slot,
                trade.instruction_index,
                trade.block_time.unix_timestamp(),
                trade.coin_token_amount,
                trade.price_coin_token_amount,
                if trade.direction == TradeDirection::Buy {
                    "buy"
                } else {
                    "sell"
                },
            )
            .as_str(),
        )
        .execute()
        .await
        .unwrap();

    Ok(())
}
