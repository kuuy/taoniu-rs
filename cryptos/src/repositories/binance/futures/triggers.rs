use diesel::prelude::*;

use crate::common::*;
use crate::schema::binance::futures::triggers::*;

#[derive(Default)]
pub struct TriggersRepository {}

impl TriggersRepository {
  pub async fn scan(ctx: Ctx) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let pool = ctx.pool.read().await;
    let mut conn = pool.get().unwrap();
    let symbols = triggers::table
      .filter(triggers::status.eq_any([1, 2]))
      .select(triggers::symbol)
      .load::<String>(&mut conn)?;
    Ok(symbols)
  }
}
