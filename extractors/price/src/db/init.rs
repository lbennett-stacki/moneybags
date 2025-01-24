use super::{
    client::{db_client, dbless_client},
    db::create_db,
};
use crate::{
    raydium::table::create_raydium_pools_table, token::table::create_tokens_table,
    trades::db::table::create_trades_table,
};
use clickhouse::Client;

pub async fn init_db() -> Client {
    let client = dbless_client();
    create_db(&client).await;

    let client = db_client();

    init_tables(&client).await;

    client
}

async fn init_tables(client: &Client) {
    let (trades_result, tokens_result, raydium_pools_result) = tokio::join!(
        create_trades_table(&client),
        create_tokens_table(&client),
        create_raydium_pools_table(&client),
    );

    trades_result.unwrap();
    tokens_result.unwrap();
    raydium_pools_result.unwrap();
}
