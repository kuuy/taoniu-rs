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
  /// indicators pivot
  Pivot,
  /// indicators atr
  Atr,
  /// indicators zlema
  Zlema,
  /// indicators ha_zlema
  HaZlema,
  /// indicators kdj
  Kdj,
  Nats(NatsCommand),
}

impl IndicatorsCommand {
  async fn pivot(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    println!("indicators pivot");
    match (IndicatorsRepository::pivot(
      ctx,
      "BTCUSDT",
      "15m",
    ).await) {
      Ok(_) => Ok(()),
      Err(e) => Err(e.into()),
    }
  }

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

  async fn zlema(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    println!("indicators zlema");
    match (IndicatorsRepository::zlema(
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

  async fn ha_zlema(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    println!("indicators ha zlema");
    match (IndicatorsRepository::ha_zlema(
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

  async fn kdj(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    println!("indicators ha kdj");
    match (IndicatorsRepository::kdj(
      ctx,
      "BTCUSDT",
      "15m",
      9,
      3,
      100,
    ).await) {
      Ok(_) => Ok(()),
      Err(e) => Err(e.into()),
    }
  }

  pub async fn run(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    match &self.commands {
      Commands::Pivot => self.pivot(ctx.clone()).await,
      Commands::Atr => self.atr(ctx.clone()).await,
      Commands::Zlema => self.zlema(ctx.clone()).await,
      Commands::HaZlema => self.ha_zlema(ctx.clone()).await,
      Commands::Kdj => self.kdj(ctx.clone()).await,
      Commands::Nats(nats) => nats.run(ctx).await,
    }
  }
}