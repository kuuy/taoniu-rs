use chrono::prelude::Utc;
use diesel::prelude::*;
use diesel::query_builder::QueryFragment;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use redis::AsyncCommands;

use crate::common::*;
use crate::config::binance::spot::config as Config;
use crate::models::binance::spot::tradings::scalping::*;
use crate::schema::binance::spot::tradings::scalping::*;
use crate::repositories::binance::ApiError;
use crate::repositories::binance::spot::tickers::*;
use crate::repositories::binance::spot::symbols::*;
use crate::repositories::binance::spot::account::*;
use crate::repositories::binance::spot::positions::*;
use crate::repositories::binance::spot::plans::*;
use crate::repositories::binance::spot::orders::*;
use crate::repositories::binance::spot::scalping::ScalpingRepository as ParentRepositoy;
use crate::repositories::binance::spot::scalping::plans::PlansRepository as ScalpingPlansRepository;

#[derive(Default)]
pub struct ScalpingRepository {}

impl ScalpingRepository {
  pub async fn find<T>(
    ctx:Ctx,
    id: T,
  ) -> Result<Option<Scalping>, Box<dyn std::error::Error>>
  where
    T: AsRef<str>
  {
    let id = id.as_ref();

    let pool = ctx.pool.read().await;
    let mut conn = pool.get().unwrap();

    match scalping::table
      .find(id)
      .select(Scalping::as_select())
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
    scalping_id: String,
    plan_id: String,
    buy_price: f64,
    sell_price: f64,
    buy_quantity: f64,
    sell_quantity: f64,
    buy_order_id: i64,
    sell_order_id: i64,
    status: i32,
    remark: String,
  ) -> Result<bool, Box<dyn std::error::Error>> {
    let pool = ctx.pool.write().await;
    let mut conn = pool.get().unwrap();

    let now = Utc::now();
    let entity = Scalping::new(
      id,
      symbol,
      scalping_id,
      plan_id,
      buy_price,
      sell_price,
      buy_quantity,
      sell_quantity,
      buy_order_id,
      sell_order_id,
      status,
      0,
      remark,
      now,
      now,
    );
    match diesel::insert_into(scalping::table)
      .values(&entity)
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
    V: diesel::AsChangeset<Target = scalping::table>,
    <V as diesel::AsChangeset>::Changeset: QueryFragment<diesel::pg::Pg>,
  {
    let pool = ctx.pool.write().await;
    let mut conn = pool.get().unwrap();
    match diesel::update(scalping::table.find(id)).set(value).execute(&mut conn) {
      Ok(effective_rows) => Ok(effective_rows > 0),
      Err(e) => Err(e.into()),
    }
  }

  pub async fn delete<T>(
    ctx: Ctx,
    id: T,
  ) -> Result<bool, Box<dyn std::error::Error>> 
  where
    T: AsRef<str>
  {
    let id = id.as_ref();

    let pool = ctx.pool.write().await;
    let mut conn = pool.get().unwrap();
    match diesel::delete(scalping::table)
      .filter(scalping::id.eq(id))
      .execute(&mut conn) {
      Ok(effective_rows) => Ok(effective_rows > 0),
      Err(e) => Err(e.into()),
    }
  }

  pub async fn place<T>(ctx: Ctx, plan_id: T) -> Result<(), Box<dyn std::error::Error>> 
  where
    T: AsRef<str>
  {
    let plan_id = plan_id.as_ref();

    let plan = match PlansRepository::find(ctx.clone(), plan_id).await {
      Ok(Some(result)) => result,
      Ok(None) => return Err(Box::from(format!("plan of {plan_id:} not exists"))),
      Err(e) => return Err(e.into()),
    };

    if plan.side != 1 {
      let _ = ScalpingPlansRepository::delete(ctx.clone(), plan_id).await;
      return Err(Box::from("only for long side plan"))
    }

    let timestamp = Utc::now().timestamp();

    if plan.interval == "1m" && plan.created_at.timestamp() < timestamp - 900
      || plan.interval == "15m" && plan.created_at.timestamp() < timestamp - 2700
      || plan.interval == "4h" && plan.created_at.timestamp() < timestamp - 5400
      || plan.interval == "1d" && plan.created_at.timestamp() < timestamp - 21600 {
      let _ = ScalpingPlansRepository::delete(ctx.clone(), plan_id).await;
      return Err(Box::from(format!("plan has been expired")))
    }

    let scalping = match ParentRepositoy::get(ctx.clone(), plan.symbol.clone()).await {
      Ok(Some(result)) => result,
      Ok(None) => return Err(Box::from(format!("scalping of {0:} not exists", plan.symbol))),
      Err(e) => return Err(e.into()),
    };

    if plan.price > scalping.price {
      let _ = ScalpingPlansRepository::delete(ctx.clone(), plan_id).await;
      return Err(Box::from(format!("plan of {0:} high then scalping price {1:}", plan.symbol, scalping.price)))
    }

    let price = match TickersRepository::price(
      ctx.clone(),
      plan.symbol.clone(),
    ).await {
      Ok(price) => price,
      Err(e) => return Err(e.into()),
    };
    let price = Decimal::from_f64(price).unwrap();

    let (tick_size, step_size) = match SymbolsRepository::filters(ctx.clone(), scalping.symbol.clone()).await {
      Ok(result) => result,
      Err(e) => return Err(e.into()),
    };
    let tick_size = Decimal::from_f64(tick_size).unwrap();
    let step_size = Decimal::from_f64(step_size).unwrap();
    let notional = dec!(10.0);

    let mut buy_price = Decimal::from_f64(plan.price).unwrap();
    if buy_price > price {
      buy_price = price;
    }
    buy_price = (buy_price / tick_size).floor() * tick_size;

    if price > buy_price {
      return Err(Box::from(format!("price of {0:} high then buy price {buy_price:}", plan.symbol)))
    }

    let entry_price = match PositionsRepository::get(ctx.clone(), scalping.symbol.clone()).await {
      Ok(Some(position)) => Decimal::from_f64(position.entry_price).unwrap(),
      Ok(None) => return Err(Box::from(format!("positions of {0:} not exists", plan.symbol))),
      Err(e) => return Err(e.into()),
    };

    if entry_price > dec!(0.0) && price > entry_price {
      let _ = ScalpingPlansRepository::delete(ctx.clone(), plan_id).await;
      return Err(Box::from(format!("plan of {0:} high then entry price {entry_price:}", plan.symbol)))
    }

    let mut sell_price = dec!(0.0);
    if plan.amount > 15.0 {
      if plan.interval == "1m" {
        sell_price = buy_price * dec!(1.0105);
      } else if plan.interval == "15m" {
        sell_price = buy_price * dec!(1.0125);
      } else if plan.interval == "4h" {
        sell_price = buy_price * dec!(1.0185);
      } else if plan.interval == "1d" {
        sell_price = buy_price * dec!(1.0385);
      }
    } else {
      if plan.interval == "1m" {
        sell_price = buy_price * dec!(1.0085);
      } else if plan.interval == "15m" {
        sell_price = buy_price * dec!(1.0105);
      } else if plan.interval == "4h" {
        sell_price = buy_price * dec!(1.012);
      } else if plan.interval == "1d" {
        sell_price = buy_price * dec!(1.0135);
      }
    }
    sell_price = (sell_price / tick_size).ceil() * tick_size;

    let buy_quantity = notional / buy_price;
    let buy_quantity = (buy_quantity / step_size).ceil() * step_size;

    if !Self::can_buy(
      ctx.clone(),
      scalping.id.clone(),
      scalping.symbol.clone(),
      buy_price.to_f64().unwrap(),
    ).await {
      return Err(Box::from(format!("scalping of {0:} can not buy now", plan.symbol)))
    }

    let (_, quote_asset) = match SymbolsRepository::pairs(
      ctx.clone(),
      plan.symbol.clone(),
    ).await {
      Ok(result) => result,
      Err(e) => return Err(e.into()),
    };

    let (free, _) = match AccountRepository::balance(
      ctx.clone(),
      &quote_asset,
    ).await {
      Ok(result) => result,
      Err(e) => return Err(e.into()),
    };

    if free < Config::SCALPING_MIN_BINANCE {
      return Err(Box::from(format!("scalping of {0:} free not enough", plan.symbol)))
    }

    let order_id = match OrdersRepository::submit(
      ctx.clone(),
      &plan.symbol[..],
      "BUY",
      buy_price.to_f64().unwrap(),
      buy_quantity.to_f64().unwrap(),
    ).await {
      Ok(result) => result,
      Err(e) => {
        if e.is::<ApiError>() {
          return Err(e.into())
        } else {
          println!("error {:?}", e);
          0
        }
      },
    };

    let id = xid::new().to_string();
    let success = match Self::create(
      ctx.clone(),
      id,
      plan.symbol.clone(),
      scalping.id,
      plan_id.to_owned(),
      buy_price.to_f64().unwrap(),
      sell_price.to_f64().unwrap(),
      buy_quantity.to_f64().unwrap(),
      buy_quantity.to_f64().unwrap(),
      order_id,
      0,
      0,
      "".to_owned(),
    ).await {
      Ok(result) => result,
      Err(e) => return Err(e.into()),
    };

    if success {
      println!("scalping of {0:} places {buy_price:} {buy_quantity:} success", plan.symbol);
    }

    Ok(())
  }

  pub async fn flush<T>(ctx: Ctx, id: T) -> Result<(), Box<dyn std::error::Error>> 
  where
    T: AsRef<str>
  {
    let _ = id.as_ref();
    let _ = ctx.pool.read().await;
    Ok(())
  }

  pub async fn can_buy<T>(
    ctx: Ctx,
    scalping_id: T,
    symbol: T,
    price: f64,
  ) -> bool
  where
    T: AsRef<str>
  {
    let scalping_id = scalping_id.as_ref();
    let symbol = symbol.as_ref();

    let mut rdb = ctx.rdb.lock().await.clone();
    let redis_key = format!("{}:{}", Config::REDIS_KEY_TRADINGS_LAST_PRICE, symbol);
    let mut cached_buy_price: f64 = match rdb.get(&redis_key).await {
      Ok(Some(result)) => result,
      _ => 0.0,
    };

    if price >= cached_buy_price * 0.9615 {
      return false
    }

    let pool = ctx.pool.write().await;
    let mut conn = pool.get().unwrap();

    let (buy_price, status) = match scalping::table
      .select((scalping::buy_price, scalping::status))
      .filter(scalping::scalping_id.eq(scalping_id))
      .filter(scalping::status.eq_any([0, 1, 2]))
      .order(scalping::buy_price.asc())
      .first::<(f64, i32)>(&mut conn) {
      Ok(result) => result,
      Err(_) => return true,
    };

    let mut is_change = false;

    if status == 0 {
      return false
    }

    if price >= buy_price * 0.9615 {
      return false
    }

    if cached_buy_price == 0.0 || cached_buy_price > buy_price {
      cached_buy_price = buy_price;
      is_change = true;
    }

    if is_change {
      let _: () = rdb.set(&redis_key, &cached_buy_price.to_be_bytes()).await.unwrap();
    }

    true
  }
}
