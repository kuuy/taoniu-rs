use redis::{aio::MultiplexedConnection, RedisError};

use crate::Env;

pub struct Rdb {}

impl Rdb {
  pub async fn new(i: u8) -> Result<MultiplexedConnection, RedisError> {
    let dsn = Env::var(format!("REDIS_{:02}_DSN", i));
    let client = redis::Client::open(dsn)?;
    let conn = client.get_multiplexed_async_connection().await?;
    Ok(conn)
  }
}
