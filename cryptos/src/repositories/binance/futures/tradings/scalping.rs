use crate::common::*;

#[derive(Default)]
pub struct ScalpingRepository {}

impl ScalpingRepository {
  pub async fn place<T>(ctx: Ctx, plan_id: T) -> Result<(), Box<dyn std::error::Error>> 
  where
    T: AsRef<str>
  {
    let _ = plan_id.as_ref();
    let _ = ctx.pool.read().await;
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
