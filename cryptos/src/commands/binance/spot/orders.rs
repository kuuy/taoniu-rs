use clap::{Parser, Args, Subcommand};

use crate::common::*;
use crate::repositories::binance::spot::orders::*;

#[derive(Parser)]
pub struct OrdersCommand {
  #[command(subcommand)]
  commands: Commands,
}

impl Default for OrdersCommand {
  fn default() -> Self {
    Self::new()
  }
}

#[derive(Subcommand)]
enum Commands {
  /// orders open
  Open,
  /// orders sync
  Sync(SyncArgs),
}

#[derive(Args)]
struct SyncArgs {
  /// symbol
  symbol: String,
  /// limit
  limit: i64,
}

impl OrdersCommand {
  pub fn new() -> Self {
    Self {
      ..Default::default()
    }
  }

  async fn open(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    println!("orders open");
    let values = OrdersRepository::open(ctx).await;
    println!("orders open {:?}", values);
    Ok(())
  }

  async fn sync(
    &self,
    ctx: Ctx,
    symbol: String,
    limit: i64,
  ) -> Result<(), Box<dyn std::error::Error>> {
    println!("orders sync");
    let values = OrdersRepository::sync(ctx, &symbol[..], 0, limit).await;
    println!("orders sync {:?}", values);
    Ok(())
  }

  pub async fn run(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    match &self.commands {
      Commands::Open => self.open(ctx.clone()).await,
      Commands::Sync(args) => self.sync(ctx.clone(), args.symbol.clone(), args.limit.clone()).await,
    }
  }
}
