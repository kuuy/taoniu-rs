use std::time::Duration;

use redis::RedisError;
use redis::aio::MultiplexedConnection;

pub struct Mutex<'a> {
  rdb: &'a mut MultiplexedConnection,
  key: &'a str,
  id: &'a str, 
}

impl<'a> Mutex<'a> {
  pub fn new(rdb: &'a mut MultiplexedConnection, key: &'a str, id: &'a str) -> Self {
    Self {
      rdb: rdb,
      key: key,
      id: id,
    }
  }

  pub async fn lock(&mut self, ttl: Duration) -> Result<bool, RedisError> {
    let result: bool = redis::cmd("SET")
      .arg(self.key)
      .arg(self.id)
      .arg("NX")
      .arg("EX")
      .arg(ttl.as_secs())
      .query_async(self.rdb)
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
    let result: bool = script.key(self.key)
      .arg(self.id)
      .invoke_async(self.rdb)
      .await?;
    Ok(result)
  }
}