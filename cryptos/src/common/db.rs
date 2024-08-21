
use std::time::Duration;

use diesel::pg::PgConnection;
use diesel::r2d2::{Builder, ConnectionManager, Pool, PoolError as R2d2Error, PooledConnection};

use crate::Env;

pub struct Db {}

impl Db {
  pub fn new(i: u8) -> Result<Pool<ConnectionManager<PgConnection>>, R2d2Error> {
    let dsn = Env::var(format!("DB_{:02}_DSN", i));
    println!("dsn {}", dsn);
    let manager = ConnectionManager::<PgConnection>::new(dsn);
    Pool::builder()
      .min_idle(Some(0))
      .max_size(100)
      .max_lifetime(Some(Duration::from_secs(600)))
      .build(manager)
  }
}