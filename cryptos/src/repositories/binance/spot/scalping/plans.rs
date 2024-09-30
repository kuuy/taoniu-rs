use diesel::prelude::*;
use diesel::query_builder::QueryFragment;
use diesel::ExpressionMethods;

use crate::common::Ctx;
use crate::models::binance::spot::scalping::plan::*;
use crate::schema::binance::spot::scalping::plans::*;

#[derive(Default)]
pub struct PlansRepository {}

impl PlansRepository {
  pub async fn scan(ctx: Ctx) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let pool = ctx.pool.read().await;
    let mut conn = pool.get().unwrap();
    let plan_ids = plans::table
      .filter(plans::status.eq(0))
      .select(plans::plan_id)
      .load::<String>(&mut conn)?;
    Ok(plan_ids)
  }

  pub async fn get<T>(
    ctx: Ctx,
    plan_id: T,
  ) -> Result<Option<Plan>, Box<dyn std::error::Error>>
  where
    T: AsRef<str>
  {
    let plan_id = plan_id.as_ref();

    let pool = ctx.pool.read().await;
    let mut conn = pool.get().unwrap();

    match plans::table
      .select(Plan::as_select())
      .filter(plans::plan_id.eq(plan_id))
      .first(&mut conn) {
        Ok(plan) => Ok(Some(plan)),
        Err(diesel::result::Error::NotFound) => Ok(None),
        Err(e) => Err(e.into()),
      }
  }

  pub async fn create(
    ctx: Ctx,
    plan_id: String,
    status: i32,
  ) -> Result<bool, Box<dyn std::error::Error>> {
    let pool = ctx.pool.write().await;
    let mut conn = pool.get().unwrap();

    let plan = Plan::new(
      plan_id,
      status,
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
}
