use redis::aio::MultiplexedConnection;
use rsmq_async::RsmqError;

pub struct Rsmq {}

impl Rsmq {
  pub async fn new(rmq: MultiplexedConnection) -> Result<rsmq_async::Rsmq, RsmqError> {
    let rsmq = rsmq_async::Rsmq::new_with_connection(rmq, false, None).await?;
    Ok(rsmq)
  }
}
