use tokio_cron::{Scheduler, Job};
use chrono::offset::Local;

use crate::common::Ctx;
use crate::tasks::binance::spot::tickers::*;

pub mod tickers;

pub struct SpotTasks<'a> {
  scheduler: &'a mut Scheduler<Local>,
}

impl<'a> SpotTasks<'a> {
  pub fn new(scheduler: &'a mut Scheduler<Local>) -> Self {
    Self {
      scheduler: scheduler,
    }
  }

  pub fn dispatch(&mut self, ctx: &'a mut Ctx<'_>) -> Result<(), Box<dyn std::error::Error>> {
    println!("tasks binance spot dispatch");
    TickersTasks::new(self.scheduler).dispatch(ctx);
    Ok(())
  }
}