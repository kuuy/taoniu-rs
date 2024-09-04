use clap::{Parser, Subcommand};

use crate::common::*;
use crate::commands::binance::spot::indicators::nats::*;
use crate::repositories::binance::spot::indicators::*;

pub mod nats;

#[derive(Parser)]
pub struct IndicatorsCommand {
  #[command(subcommand)]
  commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
  /// indicators atr
  Atr,
  Nats(NatsCommand),
}

impl IndicatorsCommand {
  async fn atr(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    println!("indicators atr");
    match (IndicatorsRepository::atr(
      ctx,
      "BTCUSDT",
      "15m",
      14,
      100,
    ).await) {
      Ok(_) => Ok(()),
      Err(e) => Err(e.into()),
    }
  }

  pub async fn run(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    match &self.commands {
      Commands::Atr => self.atr(ctx.clone()).await,
      Commands::Nats(nats) => nats.run(ctx).await,
    }
  }
}