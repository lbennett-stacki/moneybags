use crate::{
    client::CLICKHOUSE_DB_NAME,
    db::client::blocking_query,
    db::tables::{TokensRecord, TransactionsRecord},
};
use clickhouse::Client;

pub fn insert_token(client: &Client, token: TokensRecord) {
    blocking_query(
        client,
        format!(
            "INSERT INTO {}.tokens (mint_address, bonding_curve_address) VALUES ('{}', '{}')",
            CLICKHOUSE_DB_NAME, token.mint_address, token.bonding_curve_address
        )
        .as_str(),
    )
}

pub fn insert_token_pump_fun_transaction(client: &Client, token_tx: TransactionsRecord) {
    blocking_query(
        client,
        format!(
            "
        INSERT INTO {}.token_pump_fun_transactions(
            transaction_signature,
            mint_address,
            block_time,
            lamports_amount,
            token_amount,
            price_lamports
        ) VALUES ('{}', '{}', '{}', '{}', '{}', '{}')",
            CLICKHOUSE_DB_NAME,
            token_tx.transaction_signature,
            token_tx.mint_address,
            token_tx.block_time,
            token_tx.lamports_amount,
            token_tx.token_amount,
            token_tx.price_lamports
        )
        .as_str(),
    );
}
