use clap::{Parser, Args, Subcommand};
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

use crate::common::*;
use crate::commands::binance::futures::gambling::ant::*;
use crate::repositories::binance::futures::symbols::*;
use crate::repositories::binance::futures::gambling::*;

pub mod ant;

#[derive(Parser)]
pub struct GamblingCommand {
  #[command(subcommand)]
  commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
  Ant(AntCommand),
  /// gambling calc
  Calc(CalcArgs),
}

#[derive(Args)]
struct CalcArgs {
  /// symbol
  symbol: String,
  /// side
  side: i32,
  /// entry_price
  entry_price: f64,
  /// entry_quantity
  entry_quantity: f64,
}

impl GamblingCommand {
  async fn calc(
    &self,
    ctx: Ctx,
    symbol: String,
    side: i32,
    entry_price: f64,
    entry_quantity: f64,
  ) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance futures gambling calc");
    let entry_price = Decimal::from_f64(entry_price).unwrap();
    let entry_quantity = Decimal::from_f64(entry_quantity).unwrap();

    let entry_amount = entry_price * entry_quantity;

    let tick_size: f64;
    let step_size: f64;
    match SymbolsRepository::filters(ctx.clone(), symbol.clone()).await {
      Ok(data) => {
        (tick_size, step_size) = data;
      },
      Err(e) => return Err(e.into()),
    }
    let tick_size = Decimal::from_f64(tick_size).unwrap();
    let step_size = Decimal::from_f64(step_size).unwrap();

    let take_price = Decimal::from_f64(
      GamblingRepository::take_price(
        side,
        entry_price.to_f64().unwrap(),
        tick_size.to_f64().unwrap(),
      )
    ).unwrap();
    let stop_price = Decimal::from_f64(
      GamblingRepository::stop_price(
        side,
        entry_price.to_f64().unwrap(),
        tick_size.to_f64().unwrap(),
      )
    ).unwrap();

    let mut plan_price = entry_price;
    let mut plan_quantity = entry_quantity;
    let mut plan_amount = entry_amount;
    let mut plan_profit = dec!(0.0);
    let mut last_profit = dec!(0.0);
    let mut take_profit;

    loop {
      let plans = GamblingRepository::calc(
        side,
        plan_price.to_f64().unwrap(),
        plan_quantity.to_f64().unwrap(),
        tick_size.to_f64().unwrap(),
        step_size.to_f64().unwrap(),
      );
      for plan in plans.iter() {
        let plan_take_price = Decimal::from_f64(plan.take_price).unwrap();
        let plan_take_quantity = Decimal::from_f64(plan.take_quantity).unwrap();
        let plan_take_amount = Decimal::from_f64(plan.take_amount).unwrap();
 
        if plan_take_quantity < step_size {
          if side == 1 {
            last_profit = (take_price - entry_price) * plan_quantity;
          } else {
            last_profit = (entry_price - take_price) * plan_quantity;
          }
          break
        }
        if side == 1 && plan_take_price > take_price {
          last_profit = (take_price - entry_price) * plan_quantity;
          break
        }
        if side == 2 && plan_take_price < take_price {
          last_profit = (entry_price - take_price) * plan_quantity;
          break
        }
        if side == 1 {
          take_profit = (plan_take_price - entry_price) * plan_take_quantity;
        } else {
          take_profit = (entry_price - plan_take_price) * plan_take_quantity;
        }
        plan_price = plan_take_price;
        plan_quantity -= plan_take_quantity;
        plan_amount += plan_take_amount;
        plan_profit += take_profit;
        println!("plan {} {} {} {} {}", plan_take_price, plan_take_quantity, take_profit, plan_amount, plan_profit);
      }
      if plans.is_empty() || last_profit > dec!(0.0) {
        break;
      }
    }

    plan_profit += last_profit;

    if plan_profit > dec!(0.0) {
      if side == 1 {
        take_profit = (take_price - entry_price) * plan_quantity;
      } else {
        take_profit = (entry_price - take_price) * plan_quantity;
      }
      println!("plan {} {} {} {} {}", take_price, plan_quantity, take_profit, dec!(0.0), dec!(0.0));
    }

    println!("plan profit {}", plan_profit);
    println!("take price {}", take_price);
    println!("stop price {}", stop_price);

    Ok(())
  }

  pub async fn run(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    match &self.commands {
      Commands::Ant(ant) => ant.run(ctx).await,
      Commands::Calc(args) => self.calc(
        ctx.clone(),
        args.symbol.clone(),
        args.side,
        args.entry_price,
        args.entry_quantity,
      ).await,
    }
  }
}