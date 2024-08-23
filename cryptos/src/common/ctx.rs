use redis::aio::MultiplexedConnection;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub struct Ctx<'a> {
  pub rdb: &'a mut MultiplexedConnection,
  pub db: &'a mut Pool<ConnectionManager<PgConnection>>,
}