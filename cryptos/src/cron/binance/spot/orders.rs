use std::sync::Arc;

use tokio_cron::{Scheduler, Job};
use chrono::offset::Local;

use crate::common::*;

pub struct OrdersScheduler {
  scheduler: Arc<tokio::sync::Mutex<Scheduler<Local>>>,
}

impl OrdersScheduler {
  pub fn new(scheduler: Arc<tokio::sync::Mutex<Scheduler<Local>>>) -> Self {
    Self {
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

  pub async fn dispatch(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance spot orders scheduler dispatch");
    let mut scheduler = self.scheduler.lock().await;
    let context = ctx.clone();
    scheduler.add(Job::new("0 * * * * *", move || {
      Box::pin({
        let context = context.clone();
        async move {
          Self::sync(context.clone()).await;
        }
      })
    }));
    let context = ctx.clone();
    scheduler.add(Job::new("*/30 * * * * *", move || {
      Box::pin({
        let context = context.clone();
        async move {
          Self::open(context.clone()).await;
          Self::flush(context.clone()).await;
        }
      })
    }));
    Ok(())
  }
}