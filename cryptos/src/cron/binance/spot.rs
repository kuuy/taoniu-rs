use std::sync::Arc;

use tokio_cron::{Scheduler, Job};
use chrono::offset::Local;

use crate::common::*;
use crate::cron::binance::spot::tickers::*;
use crate::cron::binance::spot::depth::*;
use crate::cron::binance::spot::strategies::*;
use crate::cron::binance::spot::orders::*;

pub mod tickers;
pub mod depth;
pub mod strategies;
pub mod orders;

#[derive(Clone)]
pub struct SpotScheduler {
  scheduler: Arc<tokio::sync::Mutex<Scheduler<Local>>>,
}

impl SpotScheduler {
  pub fn new(scheduler: Scheduler<Local>) -> Self {
    Self {
      scheduler: Arc::new(tokio::sync::Mutex::new(scheduler)),
    }
  }

  pub async fn dispatch(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance spot scheduler dispatch");
    TickersScheduler::new(self.scheduler.clone()).dispatch(ctx.clone()).await;
    DepthScheduler::new(self.scheduler.clone()).dispatch(ctx.clone()).await;
    StrategiesScheduler::new(self.scheduler.clone()).dispatch(ctx.clone()).await;
    OrdersScheduler::new(self.scheduler.clone()).dispatch(ctx.clone()).await;
    Ok(())
  }
}