use tokio_cron::Scheduler;
use clap::{Parser};

use crate::common::*;
use crate::cron::binance::futures::*;

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
    println!("cron binance futures");
    let rdb = Rdb::new(2).await.unwrap();
    let rmq = Rmq::new(2).await.unwrap();
    let pool = Pool::new(2).unwrap();
    let nats = Nats::new().await.unwrap();
    let ctx = Ctx::new(rdb, rmq, pool, nats);
    let scheduler = Scheduler::local();

    let _ = FuturesScheduler::new(ctx.clone(), scheduler).dispatch().await;

    loop {
      tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    }
  }
}
