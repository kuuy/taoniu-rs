use diesel::prelude::*;

use crate::common::Ctx;
use crate::models::binance::futures::symbol::schema::dsl::*;

#[derive(Default)]
pub struct SymbolsRepository {}

impl<'a> SymbolsRepository {
  // pub fn flush(&self, _: &'a mut Ctx<'_>) -> Result<(), Box<dyn std::error::Error>> {
  //   println!("symbols flush");
  //   if 1 > 0 {
  //     return Err(Box::from("symbols repository flush failed"))
  //   }
  //   Ok(())
  // }

  pub fn count(&self, ctx: &'a mut Ctx<'_>) -> Result<i64, Box<dyn std::error::Error>> {
    let mut conn = ctx.db.get().unwrap();
    let count = schema
      .filter(status.eq("TRADING"))
      .count()
      .get_result(&mut conn)?;
    Ok(count)
  }
}
