use tokio_cron::Scheduler;
use clap::Parser;

use crate::common::*;
use crate::cron::binance::spot::*;

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
    println!("cron binance spot");
    let rdb = Rdb::new(1).await?;
    let rmq = Rmq::new(1).await?;
    let pool = Pool::new(1)?;
    let nats = Nats::new().await?;
    let ctx = Ctx::new(rdb, rmq, pool, nats);
    let scheduler = Scheduler::local();

    SpotScheduler::new(ctx.clone(), scheduler).dispatch().await?;

    loop {
      tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    }
  }
}
