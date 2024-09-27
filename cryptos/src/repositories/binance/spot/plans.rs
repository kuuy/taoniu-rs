use std::ops::Sub;
use std::time::Duration;

use chrono::{prelude::Utc, Timelike};
use diesel::prelude::*;
use diesel::query_builder::QueryFragment;
use diesel::ExpressionMethods;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

use crate::common::Ctx;
use crate::repositories::binance::spot::symbols::*;
use crate::repositories::binance::spot::strategies::*;
use crate::models::binance::spot::plan::*;
use crate::schema::binance::spot::plans::*;
use crate::queue::nats::jobs::binance::spot::plans::*;

#[derive(Default)]
pub struct PlansRepository {}

impl PlansRepository {
  pub async fn get<T>(
    ctx: Ctx,
    symbol: T,
    interval: T,
    timestamp: i64,
  ) -> Result<Option<Plan>, Box<dyn std::error::Error>>
  where
    T: AsRef<str>
  {
    let symbol = symbol.as_ref();
    let interval = interval.as_ref();

    let pool = ctx.pool.read().await;
    let mut conn = pool.get().unwrap();

    match plans::table
      .select(Plan::as_select())
      .filter(plans::symbol.eq(symbol))
      .filter(plans::interval.eq(interval))
      .filter(plans::timestamp.eq(timestamp))
      .first(&mut conn) {
        Ok(plan) => Ok(Some(plan)),
        Err(diesel::result::Error::NotFound) => Ok(None),
        Err(e) => Err(e.into()),
      }
  }

  pub async fn create(
    ctx: Ctx,
    id: String,
    symbol: String,
    interval: String,
    side: i32,
    price: f64,
    quantity: f64,
    amount: f64,
    timestamp: i64,
    status: i32,
    remark: String,
  ) -> Result<bool, Box<dyn std::error::Error>> {
    let pool = ctx.pool.write().await;
    let mut conn = pool.get().unwrap();

    let now = Utc::now();
    let plan = Plan::new(
      id,
      symbol,
      interval,
      side,
      price,
      quantity,
      amount,
      timestamp,
      status,
      remark,
      now,
      now,
    );
    match diesel::insert_into(plans::table)
      .values(&plan)
      .execute(&mut conn) {
      Ok(effective_rows) => Ok(effective_rows > 0),
      Err(e) => Err(e.into()),
    }
  }

  pub async fn update<V>(
    ctx: Ctx,
    id: String,
    value: V,
  ) -> Result<bool, Box<dyn std::error::Error>> 
  where
    V: diesel::AsChangeset<Target = plans::table>,
    <V as diesel::AsChangeset>::Changeset: QueryFragment<diesel::pg::Pg>,
  {
    let pool = ctx.pool.write().await;
    let mut conn = pool.get().unwrap();
    match diesel::update(plans::table.find(id)).set(value).execute(&mut conn) {
      Ok(effective_rows) => Ok(effective_rows > 0),
      Err(e) => Err(e.into()),
    }
  }

  pub async fn flush<T>(
    ctx: Ctx,
    symbol: T,
    interval: T,
  ) -> Result<(), Box<dyn std::error::Error>> 
  where
    T: AsRef<str>
  {
    let symbol = symbol.as_ref();
    let interval = interval.as_ref();

    let strategy = match StrategiesRepository::last(
      ctx.clone(),
      symbol,
      vec!["kdj", "ichimoku_cloud"],
      interval,
      Self::timestamp(interval) - 60000,
    ).await {
      Ok(Some(result)) => result,
      Ok(None) => return Err(Box::from(format!("strategy of {symbol:} {interval:} not exists"))),
      Err(e) => return Err(e.into()),
    };

    let mut amount = 10.0;

    for indicators in vec![
      vec!["bbands"],
      vec!["zlema", "ha_zlema"],
    ] {
      let entity = match StrategiesRepository::last(
        ctx.clone(),
        symbol,
        indicators,
        interval,
        strategy.timestamp - 14 * Self::timestep(interval),
      ).await {
        Ok(Some(result)) => result,
        Ok(None) => continue,
        Err(_) => continue,
      };
      if entity.signal != strategy.signal {
        continue
      }
      if entity.indicator == "bbands" {
        amount += 10.0;
      }
      if entity.indicator == "zlema" || entity.indicator == "ha_zlema" {
        amount += 5.0;
      }
    }

    let (tick_size, step_size) = match SymbolsRepository::filters(ctx.clone(), symbol).await {
      Ok(result) => result,
      Err(e) => return Err(e.into()),
    };
    let tick_size = Decimal::from_f64(tick_size).unwrap();
    let step_size = Decimal::from_f64(step_size).unwrap();

    let amount = Decimal::from_f64(amount).unwrap();
    let mut price = Decimal::from_f64(strategy.price).unwrap();
    price = (price / tick_size).floor() * tick_size;
    let mut quantity = amount / price;
    quantity = (quantity / step_size).ceil() * step_size;

    let plan: Option<Plan> = match Self::get(ctx.clone(), symbol, interval, strategy.timestamp).await {
      Ok(Some(result)) => Some(result),
      Ok(None) => None,
      Err(e) => return Err(e.into()),
    };

    if !plan.is_none() {
      return Err(Box::from(format!("plan {symbol:} {interval:} has been taken")))
    }

    let id = xid::new().to_string();
    let success = match Self::create(
      ctx.clone(),
      id.clone(),
      symbol.to_string(),
      interval.to_string(),
      strategy.signal,
      price.to_f64().unwrap(),
      quantity.to_f64().unwrap(),
      amount.to_f64().unwrap(),
      strategy.timestamp,
      0,
      "".to_string(),
    ).await {
      Ok(result) => result,
      Err(e) => return Err(e.into()),
    };

    if success {
      let job = PlansJob::new(ctx.clone());
      let _ = job.update(id.to_owned(), amount.to_f64().unwrap()).await;
    }

    Ok(())
  }

  pub fn timestamp<T>(interval: T) -> i64 
  where
    T: AsRef<str>
  {
    let interval = interval.as_ref();
    let mut datetime = Utc::now();
    datetime = datetime.sub(Duration::from_secs(datetime.second() as u64));
    datetime = datetime.sub(Duration::from_nanos(datetime.nanosecond() as u64));
    if interval == "15m" {
      let minutes = datetime.minute() as u64 - ((Decimal::from_u64(datetime.minute() as u64).unwrap() / dec!(15)).floor() * dec!(15)).to_u64().unwrap();
      datetime = datetime.sub(Duration::from_secs(minutes * 60));
    } else if interval == "4h" {
      let hours = datetime.hour() as u64 - ((Decimal::from_u64(datetime.hour() as u64).unwrap() / dec!(4)).floor() * dec!(4)).to_u64().unwrap();
      let minutes = datetime.minute() as u64;
      datetime = datetime.sub(Duration::from_secs(hours * 3600 + minutes * 60));
    } else if interval == "1d" {
      let hours = datetime.hour() as u64;
      let minutes = datetime.minute() as u64;
      datetime = datetime.sub(Duration::from_secs(hours * 3600 + minutes * 60));
    }
    datetime.timestamp_millis()
  }

  pub fn timestep<T>(interval: T) -> i64
  where
    T: AsRef<str>
  {
    let interval = interval.as_ref();
    if interval == "1m" {
      return 60000
    } else if interval == "15m" {
      return 900000
    } else if interval == "4h" {
      return 14400000
    }
    return 86400000
  }
}
