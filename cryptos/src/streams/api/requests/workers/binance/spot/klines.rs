use std::sync::Arc;
use std::time::Duration;
use std::collections::HashMap;

use futures_util::{stream::SplitSink, SinkExt};
use tokio::sync::Mutex;
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};

use crate::common::*;
use crate::config::binance::spot::config as Config;
use crate::streams::api::requests::payload::binance::spot::klines::*;
use crate::streams::api::*;

pub struct KlinesWorker {}

impl KlinesWorker {
  pub fn new(_: Ctx) -> Self {
    Self {}
  }

  pub async fn flush<T>(
    ctx: Ctx,
    writer: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
    payload: T,
  ) -> Result<(), Box<dyn std::error::Error>> 
  where
    T: AsRef<str>
  {
    println!("binance spot streams api requests klines flush workers process");
    let (symbol, interval, endtime, limit) = match serde_json::from_str::<KlinesFlushPayload<&str>>(payload.as_ref()) {
      Ok(result) => {
        (result.symbol, result.interval, result.endtime, result.limit)
      }
      Err(err) => return Err(err.into()),
    };

    println!("klines flush {symbol:} {interval:} {endtime:} {limit:}");

    let request_id = xid::new().to_string();
    let api_request = ApiRequest{
      id: request_id,
      method: Config::STREAMS_API_KLINES_FLUSH.to_owned(),
      params: Box::new(KlinesFlushPayload::<String>::new(symbol.to_owned(), interval.to_owned(), endtime, limit)),
    };

    // let rdb = ctx.rdb.lock().await.clone();
    // let mutex_id = xid::new().to_string();
    // let redis_lock_key = format!("{}:{}:{}", Config::LOCKS_STREAMS_API_KLINES_FLUSH, interval.to_owned(), symbol.to_owned());
    // let mut mutex = RedisMutex::new(
    //   rdb,
    //   &redis_lock_key,
    //   &mutex_id,
    // );
    // if !mutex.lock(Duration::from_secs(5)).await.unwrap() {
    //   return Err(Box::from(format!("mutex failed {}", redis_lock_key)));
    // }

    let mut writer = writer.lock_owned().await;
    let message = serde_json::to_string(&api_request).unwrap();
    println!("message {message:}");
    writer.send(Message::Text(message)).await?;

    Ok(())
  }

  pub async fn subscribe(
    &self,
    callbacks: &mut HashMap<&str, StreamFn>,
  ) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance spot plans nats workers subscribe");
    callbacks.insert(
      Config::NATS_EVENTS_API_KLINES_FLUSH,
      Box::new(|ctx, writer, payload| Box::pin(Self::flush(ctx, writer.clone(), payload))),
    );

    Ok(())
  }
}