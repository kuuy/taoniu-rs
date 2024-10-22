use std::time::Duration;

use clap::{Parser, Args, Subcommand};

use crate::common::*;
use crate::config::binance::futures::config as Config;
use crate::repositories::binance::futures::klines::*;
use crate::repositories::binance::futures::scalping::*;

#[derive(Parser)]
pub struct KlinesCommand {
  #[command(subcommand)]
  commands: Commands,
}

impl Default for KlinesCommand {
  fn default() -> Self {
    Self::new()
  }
}

#[derive(Subcommand)]
enum Commands {
  /// klines flush
  Flush(FlushArgs),
  /// klines fix
  Fix(FixArgs),
}

#[derive(Args)]
struct FlushArgs {
  /// interval 1m 15m 4h 1d
  interval: String,
  #[arg(default_value_t = 1)]
  current: u8,
}

#[derive(Args)]
struct FixArgs {
  /// interval 1m 15m 4h 1d
  interval: String,
  #[arg(default_value_t = 1)]
  current: u8,
}

impl KlinesCommand {
  pub fn new() -> Self {
    Self {
      ..Default::default()
    }
  }

  async fn flush(&self, ctx: Ctx, interval: String, current: u8) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance futures tasks klines flush {} {}", interval, current);

    let mut symbols = ScalpingRepository::scan(ctx.clone()).await.unwrap();

    if current < 1 {
      return Err(Box::from("current less then 1"))
    }

    let size = Env::usize("BINANCE_SPOT_SYMBOLS_SIZE".to_string());
    let offset = (usize::from(current) - 1) * size;
    if offset >= symbols.len() {
      return Err(Box::from("symbols out of range"))
    }

    if offset > 1 {
      let (_, items) = symbols.split_at(offset);
      symbols = items.to_vec();
    }

    if symbols.len() > size {
      let (items, _) = symbols.split_at(size);
      symbols = items.to_vec();
    }

    let limit: i64;
    if interval == "1m" {
      limit = 5;
    } else {
      limit = 1;
    }

    for symbol in symbols.iter() {
      let rdb = ctx.rdb.lock().await.clone();
      let mutex_id = xid::new().to_string();
      let redis_lock_key = format!("{}:{}:{}", Config::LOCKS_TASKS_KLINES_FLUSH, &interval, &symbol[..]);
      let mut mutex = RedisMutex::new(
        rdb,
        &redis_lock_key,
        &mutex_id,
      );
      if !mutex.lock(Duration::from_secs(30)).await.unwrap() {
        println!("mutex failed {}", redis_lock_key);
        continue
      }
      println!("binance futures klines flush {} {} {}", symbol, interval, current);
      match KlinesRepository::flush(ctx.clone(), &symbol[..], &interval, 0, limit).await {
        Ok(_) => (),
        Err(err) => println!("error: {}", err),
      }
    }

    Ok(())
  }

  async fn fix(&self, ctx: Ctx, interval: String, current: u8) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance spot tasks klines fix {} {}", interval, current);

    let mut symbols = ScalpingRepository::scan(ctx.clone()).await.unwrap();

    if current < 1 {
      return Err(Box::from("current less then 1"))
    }

    let size = Env::usize("BINANCE_SPOT_SYMBOLS_SIZE".to_string());
    let offset = (usize::from(current) - 1) * size;
    if offset >= symbols.len() {
      return Err(Box::from("symbols out of range"))
    }

    if offset > 1 {
      let (_, items) = symbols.split_at(offset);
      symbols = items.to_vec();
    }

    if symbols.len() > size {
      let (items, _) = symbols.split_at(size);
      symbols = items.to_vec();
    }

    let offset: i64;
    if &interval == "1m" {
      offset = 1440;
    } else if &interval == "15m" {
      offset = 672;
    } else if &interval == "4h" {
      offset = 126;
    } else {
      offset = 100;
    }

    for symbol in symbols.iter() {
      let rdb = ctx.rdb.lock().await.clone();
      let mutex_id = xid::new().to_string();
      let redis_lock_key = format!("{}:{}:{}", Config::LOCKS_TASKS_KLINES_FIX, &interval, &symbol[..]);
      let mut mutex = RedisMutex::new(
        rdb,
        &redis_lock_key,
        &mutex_id,
      );
      if !mutex.lock(Duration::from_secs(30)).await.unwrap() {
        println!("mutex failed {}", redis_lock_key);
        continue
      }
      println!("binance spot klines fix {} {} {}", symbol, interval, current);
      match KlinesRepository::fix(ctx.clone(), &symbol[..], &interval, offset).await {
        Ok(_) => (),
        Err(err) => println!("error: {}", err),
      }
    }

    Ok(())
  }

  pub async fn run(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    match &self.commands {
      Commands::Flush(args) => self.flush(ctx.clone(), args.interval.clone(), args.current).await,
      Commands::Fix(args) => self.fix(ctx.clone(), args.interval.clone(), args.current).await,
    }
  }
}
