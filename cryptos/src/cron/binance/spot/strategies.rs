use std::sync::Arc;

use tokio_cron::{Scheduler, Job};
use chrono::offset::Local;

use crate::common::*;

pub struct StrategiesScheduler {
  scheduler: Arc<tokio::sync::Mutex<Scheduler<Local>>>,
}

impl StrategiesScheduler {
  pub fn new(scheduler: Arc<tokio::sync::Mutex<Scheduler<Local>>>) -> Self {
    Self {
      scheduler: scheduler,
    }
  }

  pub async fn clean(ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance spot strategies scheduler clean");
    Ok(())
  }

  pub async fn dispatch(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance spot strategies scheduler dispatch");
    let mut scheduler = self.scheduler.lock().await;
    let context = ctx.clone();
    scheduler.add(Job::new("* */15 * * * *", move || {
      Box::pin({
        let context = context.clone();
        async move {
          Self::clean(context.clone()).await;
        }
      })
    }));
    Ok(())
  }
}