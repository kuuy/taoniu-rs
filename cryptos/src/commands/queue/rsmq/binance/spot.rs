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
    println!("queue rsmq binance spot");
    let mut rdb = Rdb::new(1).await.expect("redis connect failed");
    let mut db = Db::new(1).expect("db connect failed");
    let mut nats = Nats::new().await.expect("nats connect failed");
    let mut rsmq = Rsmq::new(&mut rdb).await.expect("rsmq connect failed");
    let mut ctx = Ctx{
      rdb: &mut rdb,
      db: &mut db,
      nats: &mut nats,
      rsmq: &mut rsmq,
    };
    Ok(())
  }
}
