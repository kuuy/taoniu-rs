use std::sync::{RwLock, Arc};
use std::marker::PhantomData;
use tokio::sync::Mutex;

use redis::aio::MultiplexedConnection;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

#[derive(Clone)]
pub struct Ctx {
  pub rdb: Arc<Mutex<MultiplexedConnection>>,
  pub pool: Arc<RwLock<Pool<ConnectionManager<PgConnection>>>>,
}

impl Ctx {
  pub fn new(rdb: MultiplexedConnection, pool: Pool<ConnectionManager<PgConnection>>) -> Self {
    Self {
      rdb: Arc::new(Mutex::new(rdb)),
      pool: Arc::new(RwLock::new(pool)),
    }
  }
}