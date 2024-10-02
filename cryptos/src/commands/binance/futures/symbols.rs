use std::time::Duration;

use clap::{Parser, Subcommand};

use crate::common::*;
use crate::config::binance::futures::config as Config;
use crate::repositories::binance::futures::symbols::*;

#[derive(Parser)]
pub struct SymbolsCommand {
  #[command(subcommand)]
  commands: Commands,
}

impl Default for SymbolsCommand {
  fn default() -> Self {
    Self::new()
  }
}

#[derive(Subcommand)]
enum Commands {
  /// symbols flush
  Flush,
  /// symbols count
  Count,
}

impl SymbolsCommand {
  pub fn new() -> Self {
    Self {
      ..Default::default()
    }
  }

  async fn flush(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    println!("symbols flush");
    let rdb = ctx.rdb.lock().await.clone();
    let mutex_id = xid::new().to_string();
    let mut mutex = Mutex::new(
      rdb,
      Config::LOCKS_SYMBOLS_FLUSH,
      &mutex_id,
    );
    if !mutex.lock(Duration::from_secs(600)).await.unwrap() {
      panic!("mutex failed {}", Config::LOCKS_SYMBOLS_FLUSH);
    }
    mutex.unlock().await.unwrap();
    Ok(())
  }

  async fn count(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    println!("symbols count");
    let count = SymbolsRepository::count(ctx.clone()).await.unwrap();
    println!("symbols count {}", count);
    Ok(())
  }

  pub async fn run(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    match &self.commands {
      Commands::Flush => self.flush(ctx.clone()).await,
      Commands::Count => self.count(ctx.clone()).await,
    }
  }
}
