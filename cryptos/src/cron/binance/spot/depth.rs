use std::sync::Arc;

use tokio_cron::{Scheduler, Job};
use chrono::offset::Local;

use crate::common::*;

pub struct DepthScheduler {
  ctx: Ctx,
  scheduler: Arc<tokio::sync::Mutex<Scheduler<Local>>>,
}

impl DepthScheduler {
  pub fn new(ctx: Ctx, scheduler: Arc<tokio::sync::Mutex<Scheduler<Local>>>) -> Self {
    Self {
      ctx: ctx,
      scheduler: scheduler,
    }
  }

  pub async fn flush(ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance spot depth scheduler flush");
    Ok(())
  }

  pub async fn dispatch(&self) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance spot depth scheduler dispatch");
    let mut scheduler = self.scheduler.lock().await;
    let context = self.ctx.clone();
    scheduler.add(Job::new("0 * * * * *", move || {
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