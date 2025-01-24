use clickhouse::Client;

use crate::{
    token::table::CLICKHOUSE_TOKENS_TABLE_NAME, trades::table::CLICKHOUSE_TRADES_TABLE_NAME,
};

pub const CLICKHOUSE_TRADES_CURRENT_PRICES_MATERIALIZED_VIEW_NAME: &str = "trades_current_prices";

pub async fn create_current_prices_view(client: &Client) -> Result<(), clickhouse::error::Error> {
    client
        .query(
            format!(
                "
        CREATE MATERIALIZED VIEW IF NOT EXISTS {}
        ENGINE = SummingMergeTree()
        ORDER BY coin_token_address
        POPULATE
        AS SELECT
            tra.coin_token_address,
            argMin(tra.price_coin_token_amount/(tra.coin_token_amount * pow(10, tok.decimals)), (tra.block_time, tra.slot, tra.instruction_index)) as price
        FROM {} as tra
        JOIN {} as tok ON tra.coin_token_address = tok.mint_address
        GROUP BY coin_token_address",
                CLICKHOUSE_TRADES_CURRENT_PRICES_MATERIALIZED_VIEW_NAME,
                CLICKHOUSE_TRADES_TABLE_NAME,
                CLICKHOUSE_TOKENS_TABLE_NAME
            )
            .as_str(),
        )
        .execute()
        .await?;

    Ok(())
}
