use clap::{Parser, Subcommand};

use crate::common::*;
use crate::commands::binance::spot::account::*;
use crate::commands::binance::spot::symbols::*;
use crate::commands::binance::spot::tickers::*;
use crate::commands::binance::spot::klines::*;
use crate::commands::binance::spot::indicators::*;
use crate::commands::binance::spot::strategies::*;
use crate::commands::binance::spot::plans::*;
use crate::commands::binance::spot::orders::*;
use crate::commands::binance::spot::positions::*;
use crate::commands::binance::spot::gambling::*;
use crate::commands::binance::spot::scalping::*;

pub mod account;
pub mod symbols;
pub mod tickers;
pub mod klines;
pub mod indicators;
pub mod strategies;
pub mod plans;
pub mod orders;
pub mod positions;
pub mod gambling;
pub mod scalping;
pub mod triggers;
pub mod tradings;

#[derive(Parser)]
pub struct SpotCommand {
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
  Orders(OrdersCommand),
  Positions(PositionsCommand),
  Gambling(GamblingCommand),
  Scalping(ScalpingCommand),
}

impl SpotCommand {
  pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
    let rdb = Rdb::new(1).await.unwrap();
    let rmq = Rmq::new(1).await.unwrap();
    let pool = Pool::new(1).unwrap();
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
      Commands::Orders(orders) => orders.run(ctx.clone()).await,
      Commands::Positions(positions) => positions.run(ctx.clone()).await,
      Commands::Gambling(gambling) => gambling.run(ctx.clone()).await,
      Commands::Scalping(scalping) => scalping.run(ctx.clone()).await,
    }
  }
}