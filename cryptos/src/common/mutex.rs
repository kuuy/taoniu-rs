use std::time::Duration;
use std::sync::Arc;

use redis::RedisError;
use redis::aio::MultiplexedConnection;

pub struct Mutex<T: AsRef<str>> {
  rdb: MultiplexedConnection,
  key: T,
  id: T,
}

impl <T: AsRef<str>> Mutex<T> {
  pub fn new(rdb: MultiplexedConnection, key: T, id: T) -> Self {
    Self {
      rdb: rdb,
      key: key,
      id: id,
    }
  }

  pub async fn lock(&mut self, ttl: Duration) -> Result<bool, RedisError> {
    let result: bool = redis::cmd("SET")
      .arg(self.key.as_ref())
      .arg(self.id.as_ref())
      .arg("NX")
      .arg("EX")
      .arg(ttl.as_secs())
      .query_async(&mut self.rdb)
      .await?;
    Ok(result)
  }

  pub async fn unlock(&mut self) -> Result<bool, RedisError> {
    let script = redis::Script::new(r"
      if redis.call('GET', KEYS[1]) == ARGV[1] then
        return redis.call('DEL', KEYS[1])
      else
        return 0
      end
    ");
    let result: bool = script.key(self.key.as_ref())
      .arg(self.id.as_ref())
      .invoke_async(&mut self.rdb)
      .await?;
    Ok(result)
  }
}