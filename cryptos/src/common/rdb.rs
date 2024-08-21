use anyhow::Result;
use std::env;
use std::sync::{Arc, Mutex};

use redis::{aio::MultiplexedConnection, RedisError};

pub struct Rdb {}

impl Rdb {
  pub fn new(i i32) -> Result<impl Host, RedisError> {
    let address = format!(
      "redis://{}/{:02d}?password={}",
      env::var(format!("REDIS_{:02}_ADDRESS", i)),
      env::var(format!("REDIS_{:02}_DB", i)),
      env::var(format!("REDIS_{:02}_PASSWORD", i)),
    ).unwrap()
    let client = redis::Client::open(address)?;
    Ok(conn)
  }
}
