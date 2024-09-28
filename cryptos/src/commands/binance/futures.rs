use clap::{Parser, Subcommand};

use crate::common::*;
use crate::commands::binance::futures::account::*;
use crate::commands::binance::futures::symbols::*;
use crate::commands::binance::futures::tickers::*;
use crate::commands::binance::futures::klines::*;
use crate::commands::binance::futures::indicators::*;
use crate::commands::binance::futures::strategies::*;
use crate::commands::binance::futures::plans::*;
use crate::commands::binance::futures::positions::*;
use crate::commands::binance::futures::gambling::*;
use crate::commands::binance::futures::scalping::*;
use crate::commands::binance::futures::triggers::*;
use crate::commands::binance::futures::tradings::*;

pub mod account;
pub mod symbols;
pub mod tickers;
pub mod klines;
pub mod indicators;
pub mod strategies;
pub mod plans;
pub mod positions;
pub mod gambling;
pub mod scalping;
pub mod triggers;
pub mod tradings;

#[derive(Parser)]
pub struct FuturesCommand {
  #[command(subcommand)]
  commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
  Account(AccountCommand),
  Symbols(SymbolsCommand),
  Tickers(TickersCommand),
  Klines(KlinesCommand),
  Indicators(IndicatorsCommand),
  Strategies(StrategiesCommand),
  Plans(PlansCommand),
  Positions(PositionsCommand),
  Gambling(GamblingCommand),
  Scalping(ScalpingCommand),
  Triggers(TriggersCommand),
  Tradings(TradingsCommand),
}

impl FuturesCommand {
  pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
    let rdb = Rdb::new(2).await.unwrap();
    let rmq = Rmq::new(2).await.unwrap();
    let pool = Pool::new(2).unwrap();
    let nats = Nats::new().await.unwrap();
    let ctx = Ctx::new(rdb, rmq, pool, nats);
    match &self.commands {
      Commands::Account(account) => account.run(ctx.clone()).await,
      Commands::Symbols(symbols) => symbols.run(ctx.clone()).await,
      Commands::Tickers(tickers) => tickers.run(ctx.clone()).await,
      Commands::Klines(klines) => klines.run(ctx.clone()).await,
      Commands::Indicators(indicators) => indicators.run(ctx.clone()).await,
      Commands::Strategies(strategies) => strategies.run(ctx.clone()).await,
      Commands::Plans(plans) => plans.run(ctx.clone()).await,
      Commands::Positions(positions) => positions.run(ctx.clone()).await,
      Commands::Gambling(gambling) => gambling.run(ctx.clone()).await,
      Commands::Scalping(scalping) => scalping.run(ctx.clone()).await,
      Commands::Triggers(triggers) => triggers.run(ctx.clone()).await,
      Commands::Tradings(tradings) => tradings.run(ctx.clone()).await,
    }
  }
}