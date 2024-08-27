use std::sync::{RwLock, Arc};

use tokio_cron::{Scheduler, Job};
use chrono::offset::Local;

use redis::aio::MultiplexedConnection;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

use crate::common::*;

pub struct TickersTasks<'a> {
  scheduler: &'a mut Scheduler<Local>,
}

async fn simple_async_fn() {
  println!("Hello, world!");
}

impl<'a> TickersTasks<'a> {
  pub fn new(scheduler: &'a mut Scheduler<Local>) -> Self {
    Self {
      scheduler: scheduler,
    }
  }

  pub fn flush(rdb: &mut tokio::sync::MutexGuard<'_, MultiplexedConnection>) -> Result<(), Box<dyn std::error::Error>> {
    println!("tasks binance spot tickers flush");
    Ok(())
  }

  pub fn dispatch(&mut self, ctx: &'a mut Ctx<'_>) -> Result<(), Box<dyn std::error::Error>> {
    println!("tasks binance spot tickers dispatch");
    let db = Arc::new(RwLock::new(ctx.db.clone()));
    let rdb = Arc::new(tokio::sync::Mutex::new(ctx.rdb.clone()));
    let nats = Arc::new(ctx.nats.clone());
    let rsmq = Arc::new(ctx.rsmq.clone());
    self.scheduler.add(Job::new("*/5 * * * * *", move || {
      let rdb = rdb.clone();
      async move {
        // let mut db: tokio::sync::MutexGuard<'_, Pool<ConnectionManager<PgConnection>>> = db.lock().await;
        let mut rdb: tokio::sync::MutexGuard<'_, MultiplexedConnection> = rdb.lock().await;
        let mut rsmq = Rsmq::new(&mut rdb).await.expect("rsmq connect failed");
        // let mut nats: tokio::sync::MutexGuard<'_, async_nats::Client> = nats.lock().await;
        // let mut rsmq: tokio::sync::MutexGuard<'_, rsmq_async::Rsmq> = rsmq.lock().await;
        Self::flush(&mut rdb);
      }
    }));
    Ok(())
  }
}