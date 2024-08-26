use clap::{Parser};

use crate::common::*;

#[derive(Parser)]
pub struct FuturesCommand {}

impl Default for FuturesCommand {
  fn default() -> Self {
    Self::new()
  }
}

impl<'a> FuturesCommand {
  pub fn new() -> Self {
    Self {
      ..Default::default()
    }
  }

  pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
    println!("nats queue binance futures");
    let mut rdb = Rdb::new(2).await.expect("redis connect failed");
    let mut db = Db::new(2).expect("db connect failed");
    let mut nats = Nats::new().await.expect("nats connect failed");
    let mut ctx = Ctx{
      rdb: &mut rdb,
      db: &mut db,
      nats: &mut nats,
    };
    Ok(())
  }
}
