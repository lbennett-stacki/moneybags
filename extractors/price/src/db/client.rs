use super::db::CLICKHOUSE_DB_NAME;
use crate::utils::blocking::blocking_call;
use clickhouse::Client;

pub const CLICKHOUSE_URL: &str = "http://localhost:8123";

pub fn dbless_client() -> Client {
    Client::default().with_url(CLICKHOUSE_URL).with_database("")
}

pub fn db_client() -> Client {
    dbless_client().with_database(CLICKHOUSE_DB_NAME)
}

async fn show_databases(client: &Client) -> Vec<String> {
    client.query("SHOW DATABASES").fetch_all().await.unwrap()
}

pub fn db_health_check(client: &Client) -> Vec<String> {
    blocking_call(async { show_databases(client).await })
}
