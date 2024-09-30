use std::time::Duration;

use clap::{Parser, Subcommand};

use crate::common::*;
use crate::config::binance::spot::config as Config;
use crate::repositories::binance::spot::scalping::plans::*;
use crate::repositories::binance::spot::tradings::scalping::*;

#[derive(Parser)]
pub struct ScalpingCommand {
  #[command(subcommand)]
  commands: Commands,
}

impl Default for ScalpingCommand {
  fn default() -> Self {
    Self::new()
  }
}

#[derive(Subcommand)]
enum Commands {
  /// scalping place
  Place,
  /// scalping flush
  Flush,
}

impl ScalpingCommand {
  pub fn new() -> Self {
    Self {
      ..Default::default()
    }
  }

  async fn place(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance spot tradings scalping place");
    let _ = PlansRepository::scan(ctx.clone()).await.unwrap();
    let plan_ids = vec!["crk3abqddc8eq2l64ks0"];
    for plan_id in plan_ids.iter() {
      let rdb = ctx.rdb.lock().await.clone();
      let mutex_id = xid::new().to_string();
      let redis_lock_key = format!("{}:{}", Config::LOCKS_TRADINGS_SCALPING_PLACE, plan_id);
      let mut mutex = Mutex::new(
        rdb,
        &redis_lock_key,
        &mutex_id,
      );
      if !mutex.lock(Duration::from_secs(600)).await.unwrap() {
        panic!("mutex failed {}", redis_lock_key);
      }
      match ScalpingRepository::place(ctx.clone(), plan_id).await {
        Ok(_) => {},
        Err(err) => println!("error: {}", err),
      }
      mutex.unlock().await.unwrap();
    }
    Ok(())
  }

  async fn flush(&self, _: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance spot tradings scalping flush");
    Ok(())
  }

  pub async fn run(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    match &self.commands {
      Commands::Place => self.place(ctx.clone()).await,
      Commands::Flush => self.flush(ctx.clone()).await,
    }
  }
}
