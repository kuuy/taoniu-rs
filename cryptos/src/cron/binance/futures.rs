use std::sync::Arc;

use tokio_cron::Scheduler;
use chrono::offset::Local;

use crate::common::*;
use crate::cron::binance::futures::klines::*;

pub mod klines;

#[derive(Clone)]
pub struct FuturesScheduler {
  ctx: Ctx,
  scheduler: Arc<tokio::sync::Mutex<Scheduler<Local>>>,
}

impl FuturesScheduler {
  pub fn new(ctx: Ctx, scheduler: Scheduler<Local>) -> Self {
    Self {
      ctx: ctx,
      scheduler: Arc::new(tokio::sync::Mutex::new(scheduler)),
    }
  }

  pub async fn dispatch(&self) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance futures scheduler dispatch");
    KlinesScheduler::new(self.ctx.clone(), self.scheduler.clone()).dispatch().await?;
    Ok(())
  }
}