use rust_decimal::prelude::*;
use chrono::prelude::Utc;

use crate::common::*;
use crate::repositories::binance::spot::symbols::*;
use crate::repositories::binance::spot::plans::*;
use crate::repositories::binance::spot::scalping::ScalpingRepository as ParentRepositoy;
use crate::repositories::binance::spot::scalping::plans::PlansRepository as ScalpingPlansRepository;

#[derive(Default)]
pub struct ScalpingRepository {}

impl ScalpingRepository {
  pub async fn place<T>(ctx: Ctx, plan_id: T) -> Result<(), Box<dyn std::error::Error>> 
  where
    T: AsRef<str>
  {
    let plan_id = plan_id.as_ref();
    println!("binance spot tradings scalping place plan_id {plan_id:}");
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
      //return Err(Box::from(format!("plan has been expired")))
    }

    let scalping = match ParentRepositoy::get(ctx.clone(), plan.symbol.clone()).await {
      Ok(Some(result)) => result,
      Ok(None) => return Err(Box::from(format!("scalping of {0:} not exists", plan.symbol))),
      Err(e) => return Err(e.into()),
    };

    if plan.price > scalping.price {
      let _ = ScalpingPlansRepository::delete(ctx.clone(), plan_id).await;
      return Err(Box::from(format!("plan of {0:} price too high", plan.symbol)))
    }

    let (tick_size, step_size) = match SymbolsRepository::filters(ctx.clone(), scalping.symbol).await {
      Ok(result) => result,
      Err(e) => return Err(e.into()),
    };
    let tick_size = Decimal::from_f64(tick_size).unwrap();
    let step_size = Decimal::from_f64(step_size).unwrap();
    println!("plan {0:} {1:} {tick_size:} {step_size:}", plan.id, scalping.price);

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
}
