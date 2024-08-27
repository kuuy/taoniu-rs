use redis::aio::MultiplexedConnection;
use async_nats::Client;
use rsmq_async::Rsmq;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub struct Ctx<'a> {
  pub rdb: &'a mut MultiplexedConnection,
  pub db: &'a mut Pool<ConnectionManager<PgConnection>>,
  pub nats: &'a mut Client,
  pub rsmq: &'a mut Rsmq,
}