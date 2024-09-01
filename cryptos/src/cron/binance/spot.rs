use std::sync::Arc;

use tokio_cron::{Scheduler, Job};
use chrono::offset::Local;

use crate::common::*;
use crate::cron::binance::spot::tickers::*;
use crate::cron::binance::spot::klines::*;
use crate::cron::binance::spot::depth::*;
use crate::cron::binance::spot::strategies::*;
use crate::cron::binance::spot::orders::*;

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
pub struct SpotScheduler {
  ctx: Ctx,
  scheduler: Arc<tokio::sync::Mutex<Scheduler<Local>>>,
}

impl SpotScheduler {
  pub fn new(ctx: Ctx, scheduler: Scheduler<Local>) -> Self {
    Self {
      ctx: ctx,
      scheduler: Arc::new(tokio::sync::Mutex::new(scheduler)),
    }
  }

  pub async fn dispatch(&self) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance spot scheduler dispatch");
    TickersScheduler::new(self.ctx.clone(), self.scheduler.clone()).dispatch().await;
    KlinesScheduler::new(self.ctx.clone(), self.scheduler.clone()).dispatch().await;
    DepthScheduler::new(self.ctx.clone(), self.scheduler.clone()).dispatch().await;
    StrategiesScheduler::new(self.ctx.clone(), self.scheduler.clone()).dispatch().await;
    OrdersScheduler::new(self.ctx.clone(), self.scheduler.clone()).dispatch().await;
    Ok(())
  }
}