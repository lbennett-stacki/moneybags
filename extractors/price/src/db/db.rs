use clickhouse::Client;

pub const CLICKHOUSE_DB_NAME: &str = "moneybags";

pub async fn create_db(client: &Client) {
    client
        .query(format!("CREATE DATABASE IF NOT EXISTS {}", CLICKHOUSE_DB_NAME).as_str())
        .execute()
        .await
        .unwrap()
}
