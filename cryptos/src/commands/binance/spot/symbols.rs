use std::time::Duration;

use redis::aio::MultiplexedConnection;
use clap::{Parser, Subcommand};

use crate::config::binance::spot::config as Config;
use crate::common::*;
use crate::repositories::SymbolsRepository;

#[derive(Parser)]
pub struct SymbolsCommand {
  #[command(subcommand)]
  commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
  /// symbols flush
  Flush,
}

impl<'a> SymbolsCommand {
  async fn flush(&self, rdb: &'a mut MultiplexedConnection) -> Result<(), Box<dyn std::error::Error>> {
    println!("symbols flush");
    let mutex_key = Config::LOCKS_TASKS_SYMBOLS_FLUSH;
    let mutex_id = xid::new().to_string().to_owned();
    let mut mutex = Mutex::new(
      rdb,
      mutex_key,
      &mutex_id[..],
    );
    if !mutex.lock(Duration::from_secs(600)).await? {
      panic!("mutex failed {mutex_key:?}");
    }
    Ok(())
  }

  pub async fn run(&self, rdb: &'a mut MultiplexedConnection) -> Result<(), Box<dyn std::error::Error>> {
    match &self.commands {
      Commands::Flush => self.flush(rdb).await,
    }
  }
}
