use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

use crate::repositories::binance::*;

#[derive(Default)]
pub struct GamblingRepository {}

impl GamblingRepository {
  pub fn factors(
    side: i32,
    entry_amount: f64,
  ) -> Vec<Vec<f64>> {
    let mut factors = Vec::new();
    if entry_amount < 2000.0 {
      if side == 1 {
        factors.push(vec![1.0105, 0.25]);
      } else {
        factors.push(vec![0.9895, 0.25]);
      }
    } else {
      if side == 1 {
        factors.push(vec![1.0085, 0.25]);
        factors.push(vec![1.0105, 0.5]);
      } else {
        factors.push(vec![0.9915, 0.25]);
        factors.push(vec![0.9895, 0.5]);
      }
    }
    factors
  }

  pub fn buy_quantity(
    side: i32,
    buy_amount: f64,
    entry_price: f64,
    entry_amount: f64,
  ) -> f64 {
    let mut ipart = entry_amount.floor() as i64;
    let mut places = 1;
    while ipart >= 10 {
      places += 1;
      ipart /= 10;
    }

    let buy_amount = Decimal::from_f64(buy_amount).unwrap();
    let mut entry_price = Decimal::from_f64(entry_price).unwrap();
    let mut entry_amount = Decimal::from_f64(entry_amount).unwrap();

    let mut buy_quantity = dec!(0.0);
    for _ in 0..places {
      let lost = entry_amount * dec!(0.0085);
      if side == 1 {
        entry_price = entry_price * dec!(0.9915);
        buy_quantity = (buy_amount + lost) / entry_price;
      } else {
        entry_price = entry_price * dec!(1.0085);
        buy_quantity = (buy_amount - lost) / entry_price;
      }
      entry_amount = entry_amount + lost;
    }

    buy_quantity.to_f64().unwrap()
  }

  pub fn sell_price(
    side: i32,
    entry_price: f64, 
    entry_amount: f64,
  ) -> f64 {
    let mut ipart = entry_amount.floor() as i64;
    let mut places = 1;
    while ipart >= 10 {
      places += 1;
      ipart /= 10;
    }

    let entry_price = Decimal::from_f64(entry_price).unwrap();
    let mut sell_price = dec!(0.0);
    for _ in 0..places {
      if side == 1 {
        sell_price = entry_price * dec!(1.0085);
      } else {
        sell_price = entry_price * dec!(0.9915);
      }
    }

    sell_price.to_f64().unwrap()
  }

  pub fn take_price(
    side: i32,
    entry_price: f64,
    tick_size: f64,
  ) -> f64 {
    let entry_price = Decimal::from_f64(entry_price).unwrap();
    let tick_size = Decimal::from_f64(tick_size).unwrap();

    let take_price;
    if side == 1 {
      take_price = (entry_price * dec!(1.0344) / tick_size).ceil() * tick_size;
    } else {
      take_price = (entry_price * dec!(0.9656) / tick_size).floor() * tick_size;
    }

    take_price.to_f64().unwrap()
  }

  pub fn stop_price(
    side: i32,
    entry_price: f64,
    tick_size: f64,
  ) -> f64 {
    let entry_price = Decimal::from_f64(entry_price).unwrap();
    let tick_size = Decimal::from_f64(tick_size).unwrap();

    let stop_price;
    if side == 1 {
      stop_price = (entry_price * dec!(0.9828) / tick_size).floor() * tick_size;
    } else {
      stop_price = (entry_price * dec!(1.0172) / tick_size).ceil() * tick_size;
    }

    stop_price.to_f64().unwrap()
  }

  pub fn calc(
    side: i32,
    entry_price: f64,
    entry_quantity: f64,
    tick_size: f64,
    step_size: f64,
  ) -> Vec<GamblingPlan> {
    let entry_price = Decimal::from_f64(entry_price).unwrap();
    let mut entry_quantity = Decimal::from_f64(entry_quantity).unwrap();
    let tick_size = Decimal::from_f64(tick_size).unwrap();
    let step_size = Decimal::from_f64(step_size).unwrap();

    let entry_amount = entry_price * entry_quantity;

    let mut plans = Vec::new();
    for factor in Self::factors(side, entry_amount.to_f64().unwrap()).iter() {
      let price_factor = Decimal::from_f64(factor[0]).unwrap();
      let quantity_factor = Decimal::from_f64(factor[1]).unwrap();
      let mut take_quantity = entry_quantity * quantity_factor;
      take_quantity = (take_quantity / step_size).ceil() * step_size;
      let mut take_price = entry_price * price_factor;
      if side == 1 {
        take_price = (take_price / tick_size).ceil() * tick_size;
      } else {
        take_price = (take_price / tick_size).floor() * tick_size;
      }
      if entry_quantity <= take_quantity {
        break
      }
      entry_quantity = entry_quantity - take_quantity;
      let take_amount = take_price * take_quantity;
      plans.push(GamblingPlan{
        take_price: take_price.to_f64().unwrap(),
        take_quantity: take_quantity.to_f64().unwrap(),
        take_amount: take_amount.to_f64().unwrap(),
      })
    }

    plans
  }
}
