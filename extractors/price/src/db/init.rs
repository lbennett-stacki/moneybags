use super::{
    client::{db_client, dbless_client},
    db::create_db,
};
use crate::{
    crawl_status::table::create_crawl_status_table, db::materialized::create_current_prices_view,
    raydium::table::create_raydium_pools_table, token::table::create_tokens_table,
    trades::table::create_trades_table,
};
use clickhouse::Client;

pub async fn init_db() -> Client {
    let client = dbless_client();
    create_db(&client).await;

    let client = db_client();

    init_tables(&client).await;
    init_materialized_views(&client).await;

    client
}

async fn init_tables(client: &Client) {
    let (trades_result, tokens_result, raydium_pools_result, crawl_status_result) = tokio::join!(
        create_trades_table(&client),
        create_tokens_table(&client),
        create_raydium_pools_table(&client),
        create_crawl_status_table(&client),
    );

    trades_result.unwrap();
    tokens_result.unwrap();
    raydium_pools_result.unwrap();
    crawl_status_result.unwrap();
}

async fn init_materialized_views(client: &Client) {
    let (current_prices_result,) = tokio::join!(create_current_prices_view(&client),);

    current_prices_result.unwrap();
}
