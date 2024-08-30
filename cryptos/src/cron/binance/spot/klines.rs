use std::sync::Arc;

use tokio_cron::{Scheduler, Job};
use chrono::offset::Local;

use crate::common::*;
use crate::repositories::binance::spot::klines::*;
use crate::repositories::binance::spot::scalping::*;

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

  pub async fn flush(ctx: Ctx, interval: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance spot klines scheduler flush {}", interval);
    let symbols = ScalpingRepository::scan(ctx.clone()).unwrap();
    let timestamp = KlinesRepository::timestamp(interval);
    println!("symbols {:?}", symbols);
    symbols.iter().for_each(|symbol| {
      println!("symbol {} {}", symbol, timestamp);
    });
    // array_iter.filter(|x| { let _: () = x; x == 2 });
    Ok(())
  }

  pub async fn dispatch(&self) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance spot klines scheduler dispatch");
    let mut scheduler = self.scheduler.lock().await;
    let ctx = self.ctx.clone();
    scheduler.add(Job::new("*/5 * * * * *", move || {
      Box::pin({
        let ctx = ctx.clone();
        async move {
          Self::flush(ctx.clone(), "1m").await;
        }
      })
    }));
    Ok(())
  }
}