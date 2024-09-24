use std::sync::Arc;

use tokio_cron::{Scheduler, Job};
use chrono::offset::Local;

use crate::common::*;

pub struct TickersScheduler {
  ctx: Ctx,
  scheduler: Arc<tokio::sync::Mutex<Scheduler<Local>>>,
}

impl TickersScheduler {
  pub fn new(ctx: Ctx, scheduler: Arc<tokio::sync::Mutex<Scheduler<Local>>>) -> Self {
    Self {
      ctx: ctx,
      scheduler: scheduler,
    }
  }

  pub async fn flush(ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance futures tickers scheduler flush");
    let _ = ctx.clone();
    Ok(())
  }

  pub async fn dispatch(&self) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance futures tickers scheduler dispatch");
    let mut scheduler = self.scheduler.lock().await;
    let ctx = self.ctx.clone();
    scheduler.add(Job::new("*/5 * * * * *", move || {
      Box::pin({
        let ctx = ctx.clone();
        async move {
          let _ = Self::flush(ctx.clone()).await;
        }
      })
    }));
    Ok(())
  }
}