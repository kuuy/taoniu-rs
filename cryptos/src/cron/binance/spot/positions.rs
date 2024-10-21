use std::sync::Arc;

use tokio_cron::{Scheduler, Job};
use chrono::offset::Local;

use crate::common::*;

pub struct PositionsScheduler {
  scheduler: Arc<Mutex<Scheduler<Local>>>,
}

impl PositionsScheduler {
  pub fn new(scheduler: Arc<Mutex<Scheduler<Local>>>) -> Self {
    Self {
      scheduler: scheduler,
    }
  }

  pub async fn flush(ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance spot positions scheduler flush");
    let _ = ctx.clone();
    Ok(())
  }

  pub async fn dispatch(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance spot positions scheduler dispatch");
    let mut scheduler = self.scheduler.lock().await;
    let context = ctx.clone();
    scheduler.add(Job::new("*/30 * * * * *", move || {
      Box::pin({
        let context = context.clone();
        async move {
          let _ = Self::flush(context.clone()).await;
        }
      })
    }));
    Ok(())
  }
}