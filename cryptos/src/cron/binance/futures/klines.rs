use std::sync::Arc;

use tokio_cron::{Scheduler, Job};
use chrono::offset::Local;

use crate::common::*;
use crate::queue::rsmq::jobs::binance::futures::klines::*;

pub struct KlinesScheduler {
  ctx: Ctx,
  scheduler: Arc<tokio::sync::Mutex<Scheduler<Local>>>,
}

impl KlinesScheduler {
  pub fn new(ctx: Ctx, scheduler: Arc<tokio::sync::Mutex<Scheduler<Local>>>) -> Self {
    Self {
      ctx: ctx,
      scheduler: scheduler,
    }
  }

  pub async fn sync(ctx: Ctx, interval: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance futures klines scheduler flush {}", interval);
    let job = KlinesJob::new(ctx.clone());
    job.sync(interval).await?;
    Ok(())
  }

  pub async fn dispatch(&self) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance futures klines scheduler dispatch");
    let mut scheduler = self.scheduler.lock().await;
    let ctx = self.ctx.clone();
    scheduler.add(Job::new("*/5 * * * * *", move || {
      Box::pin({
        let ctx = ctx.clone();
        async move {
          let _ = Self::sync(ctx.clone(), "1m").await;
          let _ = Self::sync(ctx.clone(), "15m").await;
          let _ = Self::sync(ctx.clone(), "4h").await;
          let _ = Self::sync(ctx.clone(), "1d").await;
        }
      })
    }));
    Ok(())
  }
}