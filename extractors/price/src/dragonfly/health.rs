use redis::{Client, RedisError};

pub fn dragonfly_health_check(client: &Client) -> Result<(), RedisError> {
    let mut conn = client.get_connection()?;

    let _: () = redis::cmd("PING").query(&mut conn)?;

    Ok(())
}
