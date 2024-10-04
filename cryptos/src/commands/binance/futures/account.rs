use std::time::Duration;

use clap::{Parser, Subcommand};

use crate::common::*;
use crate::config::binance::futures::config as Config;
use crate::commands::binance::futures::account::rsmq::*;
use crate::repositories::binance::futures::account::*;

pub mod rsmq;

#[derive(Parser)]
pub struct AccountCommand {
  #[command(subcommand)]
  commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
  /// account flush
  Flush,
  Rsmq(RsmqCommand),
}

impl AccountCommand {
  async fn flush(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance futures account flush");
    let rdb = ctx.rdb.lock().await.clone();
    let mutex_id = xid::new().to_string();
    let mut mutex = Mutex::new(
      rdb,
      Config::LOCKS_ACCOUNT_FLUSH,
      &mutex_id,
    );
    if !mutex.lock(Duration::from_secs(600)).await.unwrap() {
      return Err(Box::from(format!("mutex failed {}", Config::LOCKS_ACCOUNT_FLUSH)));
    }
    match AccountRepository::flush(ctx.clone()).await {
      Ok(_) => (),
      Err(e) => {
        mutex.unlock().await.unwrap();
        return Err(e)
      }
    }
    mutex.unlock().await.unwrap();
    Ok(())
  }

  pub async fn run(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    match &self.commands {
      Commands::Flush => self.flush(ctx.clone()).await,
      Commands::Rsmq(nats) => nats.run(ctx.clone()).await,
    }
  }
}