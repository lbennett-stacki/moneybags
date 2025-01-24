use redis::Client;

pub const REDIS_URL: &str = "redis://127.0.0.1:6379";

pub fn dragonfly_client() -> Client {
    Client::open(REDIS_URL).unwrap()
}
