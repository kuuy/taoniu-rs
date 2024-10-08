use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

use redis::aio::MultiplexedConnection;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

#[derive(Clone)]
pub struct Ctx {
  pub rdb: Arc<Mutex<MultiplexedConnection>>,
  pub rmq: Arc<Mutex<MultiplexedConnection>>,
  pub pool: Arc<RwLock<Pool<ConnectionManager<PgConnection>>>>,
  pub nats: Arc<async_nats::Client>,
}

impl Ctx {
  pub fn new(
    rdb: MultiplexedConnection,
    rmq: MultiplexedConnection,
    pool: Pool<ConnectionManager<PgConnection>>,
    nats: async_nats::Client,
  ) -> Self {
    Self {
      rdb: Arc::new(Mutex::new(rdb)),
      rmq: Arc::new(Mutex::new(rmq)),
      pool: Arc::new(RwLock::new(pool)),
      nats: Arc::new(nats),
    }
  }
}