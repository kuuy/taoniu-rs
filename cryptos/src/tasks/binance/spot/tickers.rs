use tokio_cron::{Scheduler, Job};
use chrono::offset::Local;

use crate::common::Ctx;

pub struct TickersTasks<'a> {
  scheduler: &'a mut Scheduler<Local>,
}

impl<'a> TickersTasks<'a> {
  pub fn new(scheduler: &'a mut Scheduler<Local>) -> Self {
    Self {
      scheduler: scheduler,
    }
  }

  pub async fn flush(&self) -> Result<(), Box<dyn std::error::Error>> {
    println!("tasks binance spot tickers flush");
    Ok(())
  }

  pub fn dispatch(&mut self, ctx: &'a mut Ctx<'_>) -> Result<(), Box<dyn std::error::Error>> {
    println!("tasks binance spot tickers dispatch");
    self.scheduler.add(Job::new_sync("*/5 * * * * *", move || {
      println!("Hello, world!");
    }));
    Ok(())
  }
}