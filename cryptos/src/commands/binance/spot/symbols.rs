use std::time::Duration;

use clap::{Parser, Subcommand};

use crate::common::*;
use crate::config::binance::spot::config as Config;
use crate::repositories::binance::spot::symbols::*;

#[derive(Parser)]
pub struct SymbolsCommand {
  #[clap(skip)]
  repository: SymbolsRepository,
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
      repository: SymbolsRepository{},
      ..Default::default()
    }
  }

  async fn flush(&self, ctx: AppContext) -> Result<(), Box<dyn std::error::Error>> {
    println!("symbols flush");
    let rdb = ctx.rdb.lock().await.clone();
    let mutex_id = xid::new().to_string().to_owned();
    let mut mutex = Mutex::new(
      rdb,
      Config::LOCKS_TASKS_SYMBOLS_FLUSH,
      &mutex_id[..],
    );
    if !mutex.lock(Duration::from_secs(600)).await? {
      panic!("mutex failed {}", Config::LOCKS_TASKS_SYMBOLS_FLUSH);
    }
    mutex.unlock().await?;
    Ok(())
  }

  async fn count(&self, ctx: AppContext) -> Result<(), Box<dyn std::error::Error>> {
    println!("symbols count");
    let count = self.repository.count(ctx).await.unwrap();
    println!("symbols count {}", count);
    Ok(())
  }

  pub async fn run(&self, ctx: AppContext) -> Result<(), Box<dyn std::error::Error>> {
    match &self.commands {
      Commands::Flush => self.flush(ctx).await,
      Commands::Count => self.count(ctx).await,
    }
  }
}
