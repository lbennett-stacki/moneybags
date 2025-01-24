use crate::{
    client::{CLICKHOUSE_DB_NAME, CLICKHOUSE_URL},
    db::tables::{create_token_pump_fun_transactions_table, create_tokens_table},
    utils::blocking::blocking_call,
};
use clickhouse::Client;

async fn create_db(client: &Client) {
    client
        .query(format!("CREATE DATABASE IF NOT EXISTS {}", CLICKHOUSE_DB_NAME).as_str())
        .execute()
        .await
        .unwrap()
}

pub fn blocking_query(client: &Client, query: &str) {
    blocking_call(async {
        client.query(query).execute().await.unwrap();
    })
}

fn init_dbless_client() -> Client {
    Client::default().with_url(CLICKHOUSE_URL).with_database("")
}

fn init_db_client() -> Client {
    Client::default()
        .with_url(CLICKHOUSE_URL)
        .with_database(CLICKHOUSE_DB_NAME)
}

pub fn init_db() -> Client {
    println!("init db client");

    blocking_call(async {
        println!("create db");
        let client = init_dbless_client();
        create_db(&client).await;

        println!("create tokens table");
        let client = init_db_client();
        create_tokens_table(&client).await;

        println!("create tokens tx table");
        create_token_pump_fun_transactions_table(&client).await;
    });

    init_db_client()
}

async fn show_databases(client: &Client) -> Vec<String> {
    client.query("SHOW DATABASES").fetch_all().await.unwrap()
}

pub fn health_check(client: &Client) -> Vec<String> {
    blocking_call(async { show_databases(client).await })
}
