use std::sync::Arc;

use tokio::sync::Mutex;
use tokio_cron::Scheduler;
use chrono::offset::Local;

use crate::common::*;
use crate::cron::binance::spot::klines::*;

pub mod klines;

#[derive(Clone)]
pub struct SpotScheduler {
  ctx: Ctx,
  scheduler: Arc<Mutex<Scheduler<Local>>>,
}

impl SpotScheduler {
  pub fn new(ctx: Ctx, scheduler: Scheduler<Local>) -> Self {
    Self {
      ctx: ctx,
      scheduler: Arc::new(Mutex::new(scheduler)),
    }
  }

  pub async fn dispatch(&self) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance spot scheduler dispatch");
    KlinesScheduler::new(self.ctx.clone(), self.scheduler.clone()).dispatch().await?;
    Ok(())
  }
}