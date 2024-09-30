use diesel::{Queryable, Selectable, Insertable};
use serde::{Deserialize, Serialize};

use crate::schema::binance::futures::scalping::plans::*;

#[derive(Queryable, Selectable, Insertable, Deserialize, Serialize, Debug)]
#[diesel(table_name = plans)]
pub struct Plan {
  pub plan_id: String,
  pub status: i32,
}

impl Plan {
  pub fn new(
    plan_id: String,
    status: i32,
  ) -> Self {
    Self {
      plan_id: plan_id,
      status: status,
    }
  }
}