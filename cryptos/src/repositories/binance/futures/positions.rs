use diesel::prelude::*;
use diesel::query_builder::QueryFragment;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use chrono::prelude::Utc;

use crate::common::*;
use crate::models::binance::futures::position::*;
use crate::schema::binance::futures::positions::*;

#[derive(Default)]
pub struct PositionsRepository {}

impl PositionsRepository
{
  pub async fn get<T>(
    ctx: Ctx,
    symbol: T,
    side: i32,
  ) -> Result<Option<Position>, Box<dyn std::error::Error>>
  where
    T: AsRef<str>
  {
    let symbol = symbol.as_ref();

    let pool = ctx.pool.read().await;
    let mut conn = pool.get().unwrap();

    match positions::table
      .select(Position::as_select())
      .filter(positions::symbol.eq(symbol))
      .filter(positions::side.eq(side))
      .filter(positions::status.eq(1))
      .first(&mut conn) {
      Ok(result) => Ok(Some(result)),
      Err(diesel::result::Error::NotFound) => Ok(None),
      Err(e) => Err(e.into()),
    }
  }

  pub async fn create(
    ctx: Ctx,
    id: String,
    symbol: String,
    side: i32,
    leverage: i32,
    capital: f64,
    notional: f64,
    entry_price: f64,
    entry_quantity: f64,
    timestamp: i64,
    status: i32,
  ) -> Result<bool, Box<dyn std::error::Error>> {
    let pool = ctx.pool.write().await;
    let mut conn = pool.get().unwrap();

    let now = Utc::now();
    let position = Position::new(
      id,
      symbol,
      side,
      leverage,
      capital,
      notional,
      entry_price,
      entry_quantity,
      timestamp,
      status,
      0,
      now,
      now,
    );
    match diesel::insert_into(positions::table)
      .values(&position)
      .execute(&mut conn) {
      Ok(effective_rows) => Ok(effective_rows > 0),
      Err(e) => Err(e.into()),
    }
  }

  pub async fn update<V>(
    ctx: Ctx,
    id: String,
    version: i64,
    values: V,
  ) -> Result<bool, Box<dyn std::error::Error>> 
  where
    V: diesel::AsChangeset<Target = positions::table>,
    <V as diesel::AsChangeset>::Changeset: QueryFragment<diesel::pg::Pg>,
  {
    let pool = ctx.pool.write().await;
    let mut conn = pool.get().unwrap();
    match diesel::update(positions::table.find(id))
      .filter(positions::version.eq(version))
      .set(values)
      .execute(&mut conn) {
      Ok(effective_rows) => Ok(effective_rows > 0),
      Err(e) => Err(e.into()),
    }
  }

  pub fn capital(
    capital: f64,
    entry_amount: f64,
    place: i32,
  ) -> Result<f64, Box<dyn std::error::Error>> {
    let mut capital = Decimal::from_f64(capital).unwrap();
    let step = dec!(10).powi((place - 1).into());

    let mut result = dec!(0.0);
    loop {
      let ratio = Self::ratio(
        capital.to_f64().unwrap(),
        entry_amount.to_f64().unwrap(),
      );
      if ratio == 0.0 {
        break
      }
      result = capital;
      if capital <= step {
        break
      }
      capital -= step;
    }

    if result == dec!(0.0) {
      return Err(Box::from("reach the max invest capital"))
    }

    if result < dec!(5.0) {
      return Ok(5.0)
    }

    if place > 1 {
      return Self::capital(
        (result+step).to_f64().unwrap(),
        entry_amount,
        place-1,
      )
    }

    Ok(result.to_f64().unwrap())
  }

  pub fn ratio(capital: f64, entry_amount: f64) -> f64 {
    let capital = Decimal::from_f64(capital).unwrap();
    let entry_amount = Decimal::from_f64(entry_amount).unwrap();

    let mut total_amount = dec!(0.0);
    let mut last_amount = dec!(0.0);

    let ratios = vec![0.0071, 0.0193, 0.0331, 0.0567, 0.0972, 0.1667];
    for ratio in ratios.into_iter() {
      if entry_amount == dec!(0.0) {
        return ratio
      }
      if total_amount >= entry_amount - last_amount {
        return ratio
      }
      last_amount = capital * Decimal::from_f64(ratio).unwrap();
      total_amount += last_amount;
    }

    0.0
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
      take_price = (entry_price * dec!(1.02) / tick_size).ceil() * tick_size;
    } else {
      take_price = (entry_price * dec!(0.98) / tick_size).floor() * tick_size;
    }

    take_price.to_f64().unwrap()
  }

  pub fn stop_price(
    side: i32,
    max_capital: f64,
    price: f64,
    entry_price: f64,
    entry_quantity: f64,
    tick_size: f64,
    step_size: f64,
  ) -> Result<f64, Box<dyn std::error::Error>> {
    let mut ipart = max_capital.floor() as i64;
    let mut places = 1;
    while ipart >= 10 {
      places += 1;
      ipart /= 10;
    }

    let price = Decimal::from_f64(price).unwrap();
    let mut entry_price = Decimal::from_f64(entry_price).unwrap();
    let mut entry_quantity = Decimal::from_f64(entry_quantity).unwrap();
    let mut entry_amount = entry_price * entry_quantity;
    let tick_size = Decimal::from_f64(tick_size).unwrap();
    let step_size = Decimal::from_f64(step_size).unwrap();

    let mut capital;
    let mut buy_price;
    let mut buy_quantity;
    let mut buy_amount;
    loop {
      let _ = match Self::capital(max_capital, entry_amount.to_f64().unwrap(), places) {
        Ok(result) => {
          capital = Decimal::from_f64(result).unwrap();
        },
        Err(_) => break
      };
      let ratio = Decimal::from_f64(
        Self::ratio(
          capital.to_f64().unwrap(),
          entry_amount.to_f64().unwrap(),
        ),
      ).unwrap();

      buy_amount = capital * ratio;
      if buy_amount < dec!(5.0) {
        buy_amount = dec!(5.0);
      }

      if entry_amount == dec!(0.0) {
        buy_amount = dec!(5.0);
        buy_quantity = buy_amount / price;
      } else {
        buy_quantity = Decimal::from_f64(
          Self::buy_quantity(
            side,
            buy_amount.to_f64().unwrap(),
            entry_price.to_f64().unwrap(),
            entry_amount.to_f64().unwrap(),
          ),
        ).unwrap();
      }

      buy_price = buy_amount / buy_quantity;
      if side == 1 {
        buy_price = (buy_price / tick_size).floor() * tick_size;
      } else {
        buy_price = (buy_price / tick_size).ceil() * tick_size;
      }
      buy_quantity = (buy_quantity / step_size).ceil() * step_size;
      buy_amount = buy_price * buy_quantity;
      entry_quantity += buy_quantity;
      entry_amount += buy_amount;
      entry_price = entry_amount / entry_quantity;
    }

    let stop_amount = entry_amount * dec!(0.1);
    let mut stop_price;
    if side == 1 {
      stop_price = entry_price - (stop_amount / entry_quantity);
      stop_price = (stop_price / tick_size).floor() * tick_size;
    } else {
      stop_price = entry_price + (stop_amount / entry_quantity);
      stop_price = (stop_price / tick_size).ceil() * tick_size;
    }

    Ok(stop_price.to_f64().unwrap())
  }
}
