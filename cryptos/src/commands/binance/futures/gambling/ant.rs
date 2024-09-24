use clap::{Parser, Args, Subcommand};
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

use crate::common::*;
use crate::repositories::binance::futures::symbols::*;
use crate::repositories::binance::futures::gambling::*;

#[derive(Parser)]
pub struct AntCommand {
  #[command(subcommand)]
  commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
  /// gambling ant calc
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

impl AntCommand {
  async fn calc(
    &self,
    ctx: Ctx,
    symbol: String,
    side: i32,
    entry_price: f64,
    entry_quantity: f64,
  ) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance futures gambling ant calc");
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

    let mut take_price = Decimal::from_f64(
      GamblingRepository::take_price(
        side,
        entry_price.to_f64().unwrap(),
        tick_size.to_f64().unwrap(),
      )
    ).unwrap();

    let mut plan_price = entry_price;
    let mut plan_quantity = entry_quantity;
    let mut plan_amount = entry_amount;
    let mut plan_profit = dec!(0.0);
    let mut last_price;
    let mut last_profit = dec!(0.0);
    let mut quantities = Vec::new();

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
        plan_price = plan_take_price;
        plan_quantity -= plan_take_quantity;
        quantities.push(plan.take_quantity);
      }

      if plans.is_empty() || last_profit > dec!(0.0) {
        break
      }
    }

    if plan_quantity > dec!(0.0) {
      plan_amount = dec!(0.0);
      quantities.push(plan_quantity.to_f64().unwrap());
    }

    for quantity in quantities.into_iter() {
      plan_price = take_price;
      plan_quantity = Decimal::from_f64(quantity).unwrap();
      last_price = take_price;
      take_price = Decimal::from_f64(
        GamblingRepository::take_price(
          side,
          last_price.to_f64().unwrap(),
          tick_size.to_f64().unwrap(),
        )
      ).unwrap();

      last_profit = dec!(0.0);

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
          let take_profit;
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

      if plan_quantity > dec!(0.0) {
        let take_profit;
        if side == 1 {
          take_profit = (take_price - entry_price) * plan_quantity;
        } else {
          take_profit = (entry_price - take_price) * plan_quantity;
        }
        plan_amount += take_price * plan_quantity;
        plan_profit += take_profit;
        println!("plan {} {} {} {} {}", take_price, plan_quantity, take_profit, plan_amount, plan_profit);
      }
    }

    println!("plan profit {}", plan_profit);
    println!("take price {}", take_price);

    Ok(())
  }

  pub async fn run(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    match &self.commands {
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