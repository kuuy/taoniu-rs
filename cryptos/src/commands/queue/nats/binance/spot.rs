use tokio::task::JoinSet;

use clap::{Parser};

use crate::common::*;
use crate::queue::nats::workers::binance::spot::*;

#[derive(Parser)]
pub struct SpotCommand {}

impl Default for SpotCommand {
  fn default() -> Self {
    Self::new()
  }
}

impl SpotCommand {
  pub fn new() -> Self {
    Self {
      ..Default::default()
    }
  }

  pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
    println!("queue nats binance spot");
    let rdb = Rdb::new(1).await?;
    let rmq = Rmq::new(1).await?;
    let pool = Pool::new(1)?;
    let nats = Nats::new().await?;
    let ctx = Ctx::new(rdb, rmq, pool, nats);

    let mut workers = JoinSet::new();
    SpotWorker::new(ctx.clone()).subscribe(&mut workers).await?;
    let _ = workers.join_next().await;

    Ok(())
  }
}
