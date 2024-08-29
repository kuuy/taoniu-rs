use std::sync::Arc;

use tokio_cron::{Scheduler, Job};
use chrono::offset::Local;

use crate::common::*;

pub struct TickersScheduler {
  scheduler: Arc<tokio::sync::Mutex<Scheduler<Local>>>,
}

impl TickersScheduler {
  pub fn new(scheduler: Arc<tokio::sync::Mutex<Scheduler<Local>>>) -> Self {
    Self {
      scheduler: scheduler,
    }
  }

  pub async fn flush(ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance spot tickers scheduler flush");
    Ok(())
  }

  pub async fn dispatch(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance spot tickers scheduler dispatch");
    let mut scheduler = self.scheduler.lock().await;
    let context = ctx.clone();
    scheduler.add(Job::new("*/5 * * * * *", move || {
      Box::pin({
        let context = context.clone();
        async move {
          Self::flush(context.clone()).await;
        }
      })
    }));
    Ok(())
  }
}