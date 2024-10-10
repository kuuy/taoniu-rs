use std::sync::Arc;

use tokio_cron::{Scheduler, Job};
use chrono::offset::Local;

use crate::common::*;
use crate::queue::rsmq::jobs::binance::spot::plans::*;
use crate::repositories::binance::spot::scalping::*;

pub struct PlansScheduler {
  ctx: Ctx,
  scheduler: Arc<tokio::sync::Mutex<Scheduler<Local>>>,
}

impl PlansScheduler {
  pub fn new(ctx: Ctx, scheduler: Arc<tokio::sync::Mutex<Scheduler<Local>>>) -> Self {
    Self {
      ctx: ctx,
      scheduler: scheduler,
    }
  }

  pub async fn flush(ctx: Ctx, interval: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance spot plans scheduler flush {}", interval);

    let symbols = ScalpingRepository::scan(ctx.clone()).await.unwrap();
    for symbol in symbols.iter().map(|s| s.as_ref()).collect::<Vec<&str>>() {
      let job = PlansJob::new(ctx.clone());
      job.flush(symbol, interval).await?;
    }

    Ok(())
  }

  pub async fn dispatch(&self) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance spot plans scheduler dispatch");
    let mut scheduler = self.scheduler.lock().await;
    let ctx = self.ctx.clone();
    scheduler.add(Job::new("*/5 * * * * *", move || {
      Box::pin({
        let ctx = ctx.clone();
        async move {
          let _ = Self::flush(ctx.clone(), "1m").await;
          let _ = Self::flush(ctx.clone(), "15m").await;
          let _ = Self::flush(ctx.clone(), "4h").await;
          let _ = Self::flush(ctx.clone(), "1d").await;
        }
      })
    }));
    Ok(())
  }
}