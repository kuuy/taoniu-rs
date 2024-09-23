use std::sync::Arc;

use tokio_cron::{Scheduler, Job};
use chrono::offset::Local;

use crate::common::*;
use crate::cron::binance::futures::tickers::*;
use crate::cron::binance::futures::klines::*;
use crate::cron::binance::futures::depth::*;
use crate::cron::binance::futures::strategies::*;
use crate::cron::binance::futures::orders::*;

pub mod account;
pub mod analysis;
pub mod tickers;
pub mod klines;
pub mod depth;
pub mod indicators;
pub mod strategies;
pub mod plans;
pub mod orders;
pub mod positions;
pub mod scalping;
pub mod triggers;
pub mod tradings;

#[derive(Clone)]
pub struct FuturesScheduler {
  ctx: Ctx,
  scheduler: Arc<tokio::sync::Mutex<Scheduler<Local>>>,
}

impl FuturesScheduler {
  pub fn new(ctx: Ctx, scheduler: Scheduler<Local>) -> Self {
    Self {
      ctx: ctx,
      scheduler: Arc::new(tokio::sync::Mutex::new(scheduler)),
    }
  }

  pub async fn dispatch(&self) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance futures scheduler dispatch");
    let _ = TickersScheduler::new(self.ctx.clone(), self.scheduler.clone()).dispatch().await;
    let _ = KlinesScheduler::new(self.ctx.clone(), self.scheduler.clone()).dispatch().await;
    let _ = DepthScheduler::new(self.ctx.clone(), self.scheduler.clone()).dispatch().await;
    let _ = StrategiesScheduler::new(self.ctx.clone(), self.scheduler.clone()).dispatch().await;
    let _ = OrdersScheduler::new(self.ctx.clone(), self.scheduler.clone()).dispatch().await;
    Ok(())
  }
}