use redis::aio::MultiplexedConnection;
use rsmq_async::{RsmqError, RsmqConnection};

pub struct Rsmq {}

impl<'a> Rsmq {
  pub async fn new(rdb: &'a mut MultiplexedConnection) -> Result<rsmq_async::Rsmq, RsmqError> {
    let rsmq = rsmq_async::Rsmq::new_with_connection(rdb.clone(), false, None).await?;
    Ok(rsmq)
  }
}
