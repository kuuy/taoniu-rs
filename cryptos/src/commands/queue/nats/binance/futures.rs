use tokio::task::JoinSet;
use clap::{Parser};

use crate::common::*;
use crate::queue::nats::workers::binance::futures::*;

#[derive(Parser)]
pub struct FuturesCommand {}

impl Default for FuturesCommand {
  fn default() -> Self {
    Self::new()
  }
}

impl FuturesCommand {
  pub fn new() -> Self {
    Self {
      ..Default::default()
    }
  }

  pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
    println!("queue nats binance futures");
    let rdb = Rdb::new(2).await?;
    let rmq = Rmq::new(2).await?;
    let pool = Pool::new(2)?;
    let nats = Nats::new().await?;
    let ctx = Ctx::new(rdb, rmq, pool, nats);

    let mut workers = JoinSet::new();
    FuturesWorker::new(ctx.clone()).subscribe(&mut workers).await?;
    while let Some(_) = workers.join_next().await {}

    Ok(())
  }
}
