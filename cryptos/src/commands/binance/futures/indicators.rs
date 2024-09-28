use clap::{Args, Parser, Subcommand};

use crate::common::*;
use crate::commands::binance::futures::indicators::nats::*;
use crate::repositories::binance::futures::indicators::*;

pub mod nats;

#[derive(Parser)]
pub struct IndicatorsCommand {
  #[command(subcommand)]
  commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
  /// indicators pivot
  Pivot(CmdArgs),
  /// indicators atr
  Atr(CmdArgs),
  /// indicators zlema
  Zlema(CmdArgs),
  /// indicators ha_zlema
  HaZlema(CmdArgs),
  /// indicators kdj
  Kdj(CmdArgs),
  /// indicators bbands
  Bbands(CmdArgs),
  /// indicators ichimoku cloud
  IchimokuCloud(CmdArgs),
  /// indicators volume profile
  VolumeProfile(CmdArgs),
  /// indicators andean oscillator
  AndeanOscillator(CmdArgs),
  Nats(NatsCommand),
}

#[derive(Args)]
struct CmdArgs {
  /// symbol
  symbol: String,
  /// interval
  interval: String,
}

impl IndicatorsCommand {
  async fn pivot(&self, ctx: Ctx, symbol: String, interval: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("indicators pivot");
    match IndicatorsRepository::pivot(
      ctx.clone(),
      &symbol,
      &interval,
    ).await {
      Ok(_) => Ok(()),
      Err(e) => Err(e.into()),
    }
  }

  async fn atr(&self, ctx: Ctx, symbol: String, interval: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("indicators atr");
    match IndicatorsRepository::atr(
      ctx.clone(),
      &symbol,
      &interval,
      14,
      100,
    ).await {
      Ok(_) => Ok(()),
      Err(e) => Err(e.into()),
    }
  }

  async fn zlema(&self, ctx: Ctx, symbol: String, interval: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("indicators zlema");
    match IndicatorsRepository::zlema(
      ctx.clone(),
      &symbol,
      &interval,
      14,
      100,
    ).await {
      Ok(_) => Ok(()),
      Err(e) => Err(e.into()),
    }
  }

  async fn ha_zlema(&self, ctx: Ctx, symbol: String, interval: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("indicators ha zlema");
    match IndicatorsRepository::ha_zlema(
      ctx.clone(),
      &symbol,
      &interval,
      14,
      100,
    ).await {
      Ok(_) => Ok(()),
      Err(e) => Err(e.into()),
    }
  }

  async fn kdj(&self, ctx: Ctx, symbol: String, interval: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("indicators kdj");
    match IndicatorsRepository::kdj(
      ctx.clone(),
      &symbol,
      &interval,
      9,
      3,
      100,
    ).await {
      Ok(_) => Ok(()),
      Err(e) => Err(e.into()),
    }
  }

  async fn bbands(&self, ctx: Ctx, symbol: String, interval: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("indicators bbands");
    match IndicatorsRepository::bbands(
      ctx.clone(),
      &symbol,
      &interval,
      14,
      100,
    ).await {
      Ok(_) => Ok(()),
      Err(e) => Err(e.into()),
    }
  }

  async fn ichimoku_cloud(&self, ctx: Ctx, symbol: String, interval: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("indicators ichimoku cloud");

    let tenkan_period: i32;
    let kijun_period: i32;
    let senkou_period: i32;
    let limit: i64;

    if &interval == "1m" {
      tenkan_period = 129;
      kijun_period = 374;
      senkou_period = 748;
      limit = 1440;
    } else if &interval == "15m" {
      tenkan_period = 60;
      kijun_period = 174;
      senkou_period = 349;
      limit = 672;
    } else if &interval == "4h" {
      tenkan_period = 11;
      kijun_period = 32;
      senkou_period = 65;
      limit = 126;
    } else {
      tenkan_period = 9;
      kijun_period = 26;
      senkou_period = 52;
      limit = 100;
    }

    match IndicatorsRepository::ichimoku_cloud(
      ctx.clone(),
      &symbol,
      &interval,
      tenkan_period,
      kijun_period,
      senkou_period,
      limit,
    ).await {
      Ok(_) => Ok(()),
      Err(e) => Err(e.into()),
    }
  }

  async fn volume_profile(&self, ctx: Ctx, symbol: String, interval: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("indicators volume profile");

    let limit: i64;
    if &interval == "1m" {
      limit = 1440
    } else if &interval == "15m" {
      limit = 672
    } else if &interval == "4h" {
      limit = 126
    } else {
      limit = 100
    }

    match IndicatorsRepository::volume_profile(
      ctx.clone(),
      &symbol,
      &interval,
      limit,
    ).await {
      Ok(_) => Ok(()),
      Err(e) => Err(e.into()),
    }
  }

  async fn andean_oscillator(&self, ctx: Ctx, symbol: String, interval: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("indicators andean oscillator");
    match IndicatorsRepository::andean_oscillator(
      ctx.clone(),
      &symbol,
      &interval,
      50,
      9,
      672,
    ).await {
      Ok(_) => Ok(()),
      Err(e) => Err(e.into()),
    }
  }

  pub async fn run(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    match &self.commands {
      Commands::Pivot(args) => self.pivot(
        ctx.clone(),
        args.symbol.clone(),
        args.interval.clone(),
      ).await,
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
      Commands::VolumeProfile(args) => self.volume_profile(
        ctx.clone(),
        args.symbol.clone(),
        args.interval.clone(),
      ).await,
      Commands::AndeanOscillator(args) => self.andean_oscillator(
        ctx.clone(),
        args.symbol.clone(),
        args.interval.clone(),
      ).await,
      Commands::Nats(nats) => nats.run(ctx).await,
    }
  }
}