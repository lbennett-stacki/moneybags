use crate::client::CLICKHOUSE_DB_NAME;
use clickhouse::Client;

pub struct TokensRecord {
    pub mint_address: String,
    pub bonding_curve_address: String,
}

pub async fn create_tokens_table(client: &Client) {
    client
        .query(
            format!(
                "
        CREATE TABLE IF NOT EXISTS {}.tokens(
            mint_address FixedString(44),
            bonding_curve_address FixedString(44),
            inserted_at DateTime DEFAULT now(),
        )
        ENGINE = ReplacingMergeTree(inserted_at)
        ORDER BY (mint_address, inserted_at)
    ",
                CLICKHOUSE_DB_NAME
            )
            .as_str(),
        )
        .execute()
        .await
        .unwrap()
}

pub struct TransactionsRecord {
    pub transaction_signature: String,
    pub mint_address: String,
    pub block_time: u32,
    pub block_number: u32,
    pub lamports_amount: u64,
    pub token_amount: u64,
    pub price_lamports: u64,
}

pub async fn create_token_pump_fun_transactions_table(client: &Client) {
    client
        .query(
            format!(
                "
        CREATE TABLE IF NOT EXISTS {}.token_pump_fun_transactions(
            transaction_signature FixedString(32),
            mint_address FixedString(44),
            block_time DateTime,
            lamports_amount UInt64,
            token_amount UInt64,
            direction Enum8('buy' = 1, 'sell' = 2),
            price_lamports UInt64,
            inserted_at DateTime DEFAULT now(),
        )
        ENGINE = ReplacingMergeTree(inserted_at)
        ORDER BY (transaction_signature, inserted_at, block_time)  
    ",
                CLICKHOUSE_DB_NAME
            )
            .as_str(),
        )
        .execute()
        .await
        .unwrap()
}
