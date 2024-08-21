use redis::{aio::MultiplexedConnection, RedisError};

use crate::Env;

pub struct Rdb {}

impl Rdb {
  pub async fn new(i: u8) -> Result<MultiplexedConnection, RedisError> {
    let address = format!(
      "redis://{}/{}?password={}",
      Env::var(format!("REDIS_{:02}_ADDRESS", i)),
      Env::u8(format!("REDIS_{:02}_DB", i)),
      Env::var(format!("REDIS_{:02}_PASSWORD", i)),
    );
    let client = redis::Client::open(address)?;
    let conn = client.get_multiplexed_async_connection().await?;
    Ok(conn)
  }
}
