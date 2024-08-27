
use std::time::Duration;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, PoolError};

use crate::Env;

pub struct Pool {}

impl Pool {
  pub fn new(i: u8) -> Result<diesel::r2d2::Pool<ConnectionManager<PgConnection>>, PoolError> {
    let dsn = Env::var(format!("DB_{:02}_DSN", i));
    println!("dsn {dsn:}");
    let manager = ConnectionManager::<PgConnection>::new(dsn);
    diesel::r2d2::Pool::builder()
      .min_idle(Some(0))
      .max_size(100)
      .max_lifetime(Some(Duration::from_secs(600)))
      .build(manager)
  }
}