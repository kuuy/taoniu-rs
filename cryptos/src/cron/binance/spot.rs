use tokio_cron::{Scheduler, Job};
use chrono::offset::Local;

use crate::common::Ctx;
use crate::cron::binance::spot::tickers::*;

pub mod tickers;

pub struct SpotScheduler<'a> {
  scheduler: &'a mut Scheduler<Local>,
}

impl<'a> SpotScheduler<'a> {
  pub fn new(scheduler: &'a mut Scheduler<Local>) -> Self {
    Self {
      scheduler: scheduler,
    }
  }

  pub fn dispatch(&mut self, ctx: &'a mut Ctx<'_>) -> Result<(), Box<dyn std::error::Error>> {
    println!("tasks binance spot dispatch");
    TickersScheduler::new(self.scheduler).dispatch(ctx);
    Ok(())
  }
}