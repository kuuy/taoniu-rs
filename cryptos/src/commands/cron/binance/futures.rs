use tokio::sync::Mutex;
use tokio_cron::{Scheduler, Job};
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
    let rdb = Rdb::new(2).await.unwrap();
    let rmq = Rmq::new(2).await.unwrap();
    let pool = Pool::new(2).unwrap();
    let nats = Nats::new().await.unwrap();
    let ctx = Ctx::new(rdb, rmq, pool, nats);
    // let scheduler = Scheduler::local();

    // scheduler.add(Job::new_sync("*/1 * * * * *", move || {
    //   println!("Hello, world!");
    // }));

    // loop {
    //   tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    // }

    Ok(())
  }
}
