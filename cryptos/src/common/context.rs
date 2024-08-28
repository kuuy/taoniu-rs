use std::sync::{RwLock, Arc};
use tokio::sync::Mutex;

use redis::aio::MultiplexedConnection;
use async_nats::Client;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

#[derive(Clone)]
pub struct ApiContext {
  pub rdb: Arc<Mutex<MultiplexedConnection>>,
  pub db_pool: Arc<RwLock<Pool<ConnectionManager<PgConnection>>>>,
}

#[derive(Clone)]
pub struct AppContext {
  pub rdb: Arc<Mutex<MultiplexedConnection>>,
  pub pool: Arc<RwLock<Pool<ConnectionManager<PgConnection>>>>,
}

impl AppContext {
  pub fn new(rdb: MultiplexedConnection, pool: Pool<ConnectionManager<PgConnection>>) -> Self {
    Self {
      rdb: Arc::new(Mutex::new(rdb)),
      pool: Arc::new(RwLock::new(pool)),
    }
  }
}