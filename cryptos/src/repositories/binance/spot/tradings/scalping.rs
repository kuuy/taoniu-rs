use crate::common::*;
use crate::repositories::binance::spot::plans::*;

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
      
    }
    println!("plan {0:}", plan.id);

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
