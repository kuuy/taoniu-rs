use futures_util::StreamExt;
use rsmq_async::{RsmqMessage, RsmqConnection};

use crate::common::*;
use crate::config::binance::spot::config as Config;
use crate::queue::rsmq::payload::binance::spot::klines::*;
use crate::repositories::binance::spot::scalping::*;
use crate::repositories::binance::spot::klines::*;

pub struct KlinesWorker {
  ctx: Ctx,
}

impl KlinesWorker {
  pub fn new(ctx: Ctx) -> Self {
    Self {
      ctx: ctx,
    }
  }

  pub async fn flush<T>(ctx: Ctx, interval: T) -> Result<(), Box<dyn std::error::Error>> 
  where
    T: AsRef<str>
  {
    let symbols = ScalpingRepository::scan(ctx.clone()).await.unwrap();
    let interval = interval.as_ref();
    let timestamp = KlinesRepository::timestamp(interval);
    KlinesRepository::flush(ctx.clone(), symbols.iter().map(String::as_ref).collect(), interval, timestamp).await;
    println!("rsmq workers binance spot flush {symbols:?} {interval:} {timestamp:}");
    Ok(())
  }

  pub async fn subscribe(&self) -> Result<(), Box<dyn std::error::Error>> 
  {
    println!("binance spot klines rsmq workers subscribe");
    let ctx = self.ctx.clone();
    tokio::spawn(Box::pin({
      let rmq = ctx.rmq.lock().await.clone();
      async move {
        let mut client = Rsmq::new(rmq.clone()).await.unwrap();
        loop {
          let _ = match client.receive_message::<String>(Config::RSMQ_QUEUE_KLINES, None).await {
            Ok(Some(message)) => {
              println!("message received: {:?}", message);
              let (action, content) = serde_json::from_slice::<(String, String)>(message.message.as_bytes()).unwrap();
              match action.as_str() {
                Config::RSMQ_JOBS_KLINES_FLUSH => {
                  let payload = serde_json::from_slice::<KlinesFlushPayload<&str>>(content.as_bytes()).unwrap();
                  Self::flush(ctx.clone(), payload.interval).await;
                }
                _ => {},
              };
              println!("action {action:} payload {content:}");
              // client.delete_message(Config::RSMQ_QUEUE_KLINES, &message.id).await;
            },
            Ok(None) => {
              tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            }
            Err(_) => {}
          };
        }
      }
    }));
    Ok(())
  }
}