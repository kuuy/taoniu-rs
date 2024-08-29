use std::sync::Arc;

use tokio_cron::{Scheduler, Job};
use chrono::offset::Local;

use crate::common::*;

pub struct OrdersScheduler {
  ctx: Ctx,
  scheduler: Arc<tokio::sync::Mutex<Scheduler<Local>>>,
}

impl OrdersScheduler {
  pub fn new(ctx: Ctx, scheduler: Arc<tokio::sync::Mutex<Scheduler<Local>>>) -> Self {
    Self {
      ctx: ctx,
      scheduler: scheduler,
    }
  }

  pub async fn open(ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance spot orders scheduler open");
    Ok(())
  }

  pub async fn flush(ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance spot orders scheduler flush");
    Ok(())
  }

  pub async fn sync(ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance spot orders scheduler sync");
    Ok(())
  }

  pub async fn dispatch(&self) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance spot orders scheduler dispatch");
    let mut scheduler = self.scheduler.lock().await;
    let ctx = self.ctx.clone();
    scheduler.add(Job::new("0 * * * * *", move || {
      Box::pin({
        let ctx = ctx.clone();
        async move {
          Self::sync(ctx.clone()).await;
        }
      })
    }));
    let ctx = self.ctx.clone();
    scheduler.add(Job::new("*/30 * * * * *", move || {
      Box::pin({
        let ctx = ctx.clone();
        async move {
          Self::open(ctx.clone()).await;
          Self::flush(ctx.clone()).await;
        }
      })
    }));
    Ok(())
  }
}