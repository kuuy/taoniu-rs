use tokio_cron::{Scheduler, Job};
use chrono::offset::Local;

use crate::common::*;
use crate::cron::binance::spot::tickers::*;

pub mod tickers;

pub struct SpotScheduler {
  scheduler: Scheduler<Local>,
}

impl SpotScheduler {
  pub fn new(scheduler: Scheduler<Local>) -> Self {
    Self {
      scheduler: scheduler,
    }
  }

  pub fn dispatch(&mut self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    println!("tasks binance spot dispatch");
    TickersScheduler::new(&mut self.scheduler).dispatch(ctx);
    Ok(())
  }
}