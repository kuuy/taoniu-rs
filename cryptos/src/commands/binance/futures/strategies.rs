use clap::{Args, Parser, Subcommand};

use crate::common::*;
use crate::commands::binance::futures::strategies::nats::*;
use crate::repositories::binance::futures::strategies::*;

pub mod nats;

#[derive(Parser)]
pub struct StrategiesCommand {
  #[command(subcommand)]
  commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
  /// strategies atr
  Atr(CmdArgs),
  /// strategies zlema
  Zlema(CmdArgs),
  /// strategies ha_zlema
  HaZlema(CmdArgs),
  /// strategies kdj
  Kdj(CmdArgs),
  /// strategies bbands
  Bbands(CmdArgs),
  /// strategies ichimoku cloud
  IchimokuCloud(CmdArgs),
  Nats(NatsCommand),
}

#[derive(Args)]
struct CmdArgs {
  /// symbol
  symbol: String,
  /// interval
  interval: String,
}

impl StrategiesCommand {
  async fn atr(&self, ctx: Ctx, symbol: String, interval: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("strategies atr");
    match StrategiesRepository::atr(
      ctx,
      &symbol,
      &interval,
    ).await {
      Ok(_) => Ok(()),
      Err(e) => Err(e.into()),
    }
  }

  async fn zlema(&self, ctx: Ctx, symbol: String, interval: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("strategies zlema");
    match StrategiesRepository::zlema(
      ctx,
      &symbol,
      &interval,
    ).await {
      Ok(_) => Ok(()),
      Err(e) => Err(e.into()),
    }
  }

  async fn ha_zlema(&self, ctx: Ctx, symbol: String, interval: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("strategies ha zlema");
    match StrategiesRepository::ha_zlema(
      ctx,
      &symbol,
      &interval,
    ).await {
      Ok(_) => Ok(()),
      Err(e) => Err(e.into()),
    }
  }

  async fn kdj(&self, ctx: Ctx, symbol: String, interval: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("strategies kdj");
    match StrategiesRepository::kdj(
      ctx,
      &symbol,
      &interval,
    ).await {
      Ok(_) => Ok(()),
      Err(e) => Err(e.into()),
    }
  }

  async fn bbands(&self, ctx: Ctx, symbol: String, interval: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("strategies bbands");
    match StrategiesRepository::bbands(
      ctx,
      &symbol,
      &interval,
    ).await {
      Ok(_) => Ok(()),
      Err(e) => Err(e.into()),
    }
  }

  async fn ichimoku_cloud(&self, ctx: Ctx, symbol: String, interval: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("strategies ichimoku cloud");
    match StrategiesRepository::ichimoku_cloud(
      ctx,
      &symbol,
      &interval,
    ).await {
      Ok(_) => Ok(()),
      Err(e) => Err(e.into()),
    }
  }

  pub async fn run(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    match &self.commands {
      Commands::Atr(args) => self.atr(
        ctx.clone(),
        args.symbol.clone(),
        args.interval.clone(),
      ).await,
      Commands::Zlema(args) => self.zlema(
        ctx.clone(),
        args.symbol.clone(),
        args.interval.clone(),
      ).await,
      Commands::HaZlema(args) => self.ha_zlema(
        ctx.clone(),
        args.symbol.clone(),
        args.interval.clone(),
      ).await,
      Commands::Kdj(args) => self.kdj(
        ctx.clone(),
        args.symbol.clone(),
        args.interval.clone(),
      ).await,
      Commands::Bbands(args) => self.bbands(
        ctx.clone(),
        args.symbol.clone(),
        args.interval.clone(),
      ).await,
      Commands::IchimokuCloud(args) => self.ichimoku_cloud(
        ctx.clone(),
        args.symbol.clone(),
        args.interval.clone(),
      ).await,
      Commands::Nats(nats) => nats.run(ctx).await,
    }
  }
}