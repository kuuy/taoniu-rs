use clap::{Args, Parser, Subcommand};

use crate::common::*;
use crate::repositories::binance::spot::plans::*;

#[derive(Parser)]
pub struct PlansCommand {
  #[command(subcommand)]
  commands: Commands,
}

impl Default for PlansCommand {
  fn default() -> Self {
    Self::new()
  }
}

#[derive(Subcommand)]
enum Commands {
  /// plans flush
  Flush(FlushArgs),
}

#[derive(Args)]
struct FlushArgs {
  /// symbol
  symbol: String,
  /// interval
  interval: String,
}

impl PlansCommand {
  pub fn new() -> Self {
    Self {
      ..Default::default()
    }
  }

  async fn flush(&self, ctx: Ctx, symbol: String, interval: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("plans flush");
    match PlansRepository::flush(ctx.clone(), &symbol, &interval).await {
      Ok(_) => Ok(()),
      Err(e) => Err(e.into()),
    }
  }

  pub async fn run(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    match &self.commands {
      Commands::Flush(args) => self.flush(
        ctx.clone(),
        args.symbol.clone(),
        args.interval.clone(),
      ).await,
    }
  }
}
