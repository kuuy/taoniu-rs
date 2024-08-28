use clap::{Parser};

use crate::common::*;

#[derive(Parser)]
pub struct SpotCommand {}

impl Default for SpotCommand {
  fn default() -> Self {
    Self::new()
  }
}

impl<'a> SpotCommand {
  pub fn new() -> Self {
    Self {
      ..Default::default()
    }
  }

  pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
    println!("queue nats binance spot");
    let rdb = Rdb::new(1).await.unwrap();
    let pool = Pool::new(1).unwrap();
    let ctx = Ctx::new(rdb, pool);
    Ok(())
  }
}
