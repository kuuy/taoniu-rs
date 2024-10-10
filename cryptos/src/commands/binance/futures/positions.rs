use clap::{Parser, Args, Subcommand};
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

use crate::common::*;
use crate::repositories::binance::futures::symbols::*;
use crate::repositories::binance::futures::positions::*;

#[derive(Parser)]
pub struct PositionsCommand {
  #[command(subcommand)]
  commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
  /// positions calc
  Calc(CalcArgs),
}

#[derive(Args)]
struct CalcArgs {
  /// symbol
  symbol: String,
  /// margin
  margin: f64,
  /// leverage
  leverage: i32,
  /// side
  side: i32,
  /// entry_price
  entry_price: f64,
  /// entry_quantity
  entry_quantity: f64,
}

impl PositionsCommand {
  async fn calc(
    &self,
    ctx: Ctx,
    symbol: String,
    margin: f64,
    leverage: i32,
    side: i32,
    entry_price: f64,
    entry_quantity: f64,
  ) -> Result<(), Box<dyn std::error::Error>> {
    println!("biannce futures positions calc");
    let margin = Decimal::from_f64(margin).unwrap();
    let leverage = Decimal::from_i32(leverage).unwrap();
    let mut entry_price = Decimal::from_f64(entry_price).unwrap();
    let mut entry_quantity = Decimal::from_f64(entry_quantity).unwrap();

    let max_capital = margin * leverage;
    let mut entry_amount = entry_price * entry_quantity;

    let tick_size: f64;
    let step_size: f64;
    match SymbolsRepository::filters(ctx.clone(), symbol.clone()).await {
      Ok(data) => {
        (tick_size, step_size, _) = data;
      }
      Err(err) => return Err(err.into()),
    }
    let tick_size = Decimal::from_f64(tick_size).unwrap();
    let step_size = Decimal::from_f64(step_size).unwrap();

    let mut buy_price;
    let mut buy_quantity;
    let mut buy_amount;
    let mut sell_price;
    let take_price;

    if entry_amount < dec!(5.0) {
      buy_price = entry_price;
      buy_quantity = dec!(5.0) / buy_price;
      buy_quantity = (buy_quantity / step_size).ceil() * step_size;
      buy_amount = buy_price * buy_quantity;
      entry_quantity = buy_quantity;
      entry_amount = buy_amount;
      take_price = Decimal::from_f64(
        PositionsRepository::take_price(
          side,
          entry_price.to_f64().unwrap(),
          tick_size.to_f64().unwrap(),
        ),
      ).unwrap();
    } else {
      take_price = Decimal::from_f64(
        PositionsRepository::take_price(
          side,
          entry_price.to_f64().unwrap(),
          tick_size.to_f64().unwrap(),
        ),
      ).unwrap();
    }

    let mut ipart: i64 = max_capital.floor().to_i64().unwrap();
    let mut places: i32 = 1;
    while ipart >= 10 {
      places += 1;
      ipart /= 10;
    }

    loop {
      let capital;
      match PositionsRepository::capital(
        max_capital.to_f64().unwrap(),
        entry_amount.to_f64().unwrap(),
        places,
      ) {
        Ok(result) => {
          capital = Decimal::from_f64(result).unwrap();
        }
        Err(_) => break
      };
      let ratio = Decimal::from_f64(
        PositionsRepository::ratio(
          capital.to_f64().unwrap(),
          entry_amount.to_f64().unwrap(),
        ),
      ).unwrap();

      buy_amount = capital * ratio;
      if buy_amount < dec!(5.0) {
        buy_amount = dec!(5.0);
      }

      buy_quantity = Decimal::from_f64(
        PositionsRepository::buy_quantity(
          side,
          buy_amount.to_f64().unwrap(),
          entry_price.to_f64().unwrap(),
          entry_amount.to_f64().unwrap(),
        ),
      ).unwrap();
      buy_price = buy_amount / buy_quantity;
      if side == 1 {
        buy_price = (buy_price / tick_size).floor() * tick_size;
      } else {
        buy_price = (buy_price / tick_size).ceil() * tick_size;
      }
      buy_quantity = (buy_quantity / step_size).ceil() * step_size;
      buy_amount = buy_price * buy_quantity;
      entry_quantity = entry_quantity + buy_quantity;
      entry_amount = entry_amount + buy_amount;
      entry_price = entry_amount / entry_quantity;
      sell_price = Decimal::from_f64(
        PositionsRepository::sell_price(
          side,
          entry_price.to_f64().unwrap(),
          entry_amount.to_f64().unwrap(),
        ),
      ).unwrap();
      if side == 1 {
        sell_price = (sell_price / tick_size).ceil() * tick_size;
      } else {
        sell_price = (sell_price / tick_size).floor() * tick_size;
      }
      println!("buy {} {} {} {} {}", buy_price, buy_quantity, buy_amount, sell_price, entry_price)
    }

    let stop_amount = (entry_amount / leverage) * dec!(0.1);
    let mut stop_price;
    if side == 1 {
      stop_price = entry_price - stop_amount / entry_quantity;
      stop_price = (stop_price / tick_size).floor() * tick_size;
    } else {
      stop_price = entry_price + stop_amount / entry_quantity;
      stop_price = (stop_price / tick_size).ceil() * tick_size;
    }

    println!("take price {}", take_price);
    println!("stop price {}", stop_price);

    Ok(())
  }

  pub async fn run(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    match &self.commands {
      Commands::Calc(args) => self.calc(
        ctx.clone(),
        args.symbol.clone(),
        args.margin,
        args.leverage,
        args.side,
        args.entry_price,
        args.entry_quantity,
      ).await,
    }
  }
}