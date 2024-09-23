use std::sync::Arc;

use tokio_cron::{Scheduler, Job};
use chrono::offset::Local;

use crate::common::*;

pub struct StrategiesScheduler {
  ctx: Ctx,
  scheduler: Arc<tokio::sync::Mutex<Scheduler<Local>>>,
}

impl StrategiesScheduler {
  pub fn new(ctx: Ctx, scheduler: Arc<tokio::sync::Mutex<Scheduler<Local>>>) -> Self {
    Self {
      ctx: ctx,
      scheduler: scheduler,
    }
  }

  pub async fn clean(ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance spot strategies scheduler clean");
    Ok(())
  }

  pub async fn dispatch(&self) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance spot strategies scheduler dispatch");
    let mut scheduler = self.scheduler.lock().await;
    let ctx = self.ctx.clone();
    scheduler.add(Job::new("* */15 * * * *", move || {
      Box::pin({
        let ctx = ctx.clone();
        async move {
          let _ = Self::clean(ctx.clone()).await;
        }
      })
    }));
    Ok(())
  }
}