use std::sync::Arc;

use chrono::prelude::Utc;
use rust_decimal::prelude::*;
use redis::AsyncCommands;
use redis::aio::MultiplexedConnection;
use futures_util::stream::StreamExt;
use serde::{Deserialize, Deserializer};
use tokio_tungstenite::connect_async;
use clap::{Parser};

use crate::common::*;
use crate::config::binance::spot::config as Config;
use crate::repositories::binance::spot::scalping::*;

#[derive(Parser)]
pub struct KlinesCommand {
  #[arg(skip)]
  scalping_repository: ScalpingRepository,
  interval: String,
  #[arg(default_value_t = 1)]
  current: u8,
}

impl Default for KlinesCommand {
  fn default() -> Self {
    Self::new()
  }
}

#[derive(Deserialize)]
struct KlineEvent {
  #[serde(alias = "data")]
  data: KlineData,
}

#[derive(Deserialize)]
struct KlineData {
  #[serde(alias = "k")]
  message: KlineMessage,
}

#[derive(Deserialize)]
struct KlineMessage {
  #[serde(alias = "s")]
  symbol: String,
  #[serde(alias = "i")]
  interval: String,
  #[serde(alias = "o", deserialize_with = "to_f64")]
  open: f64,
  #[serde(alias = "c", deserialize_with = "to_f64")]
  price: f64,
  #[serde(alias = "h", deserialize_with = "to_f64")]
  high: f64,
  #[serde(alias = "l", deserialize_with = "to_f64")]
  low: f64,
  #[serde(alias = "v", deserialize_with = "to_f64")]
  volume: f64,
  #[serde(alias = "q", deserialize_with = "to_f64")]
  quota: f64,
  #[serde(alias = "t")]
  timestamp: i64,
}

fn to_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
  D: Deserializer<'de>,
{
  let s: &str = Deserialize::deserialize(deserializer)?;
  s.parse::<f64>().map_err(serde::de::Error::custom)
}

impl<'a> KlinesCommand {
  pub fn new() -> Self {
    Self {
      scalping_repository: ScalpingRepository{},
      ..Default::default()
    }
  }

  async fn process(rdb: &mut tokio::sync::MutexGuard<'_, MultiplexedConnection>, message: KlineMessage) -> Result<(), Box<dyn std::error::Error>> {
    println!("process message {} {}", message.symbol, message.timestamp);
    let open = Decimal::from_f64(message.open).unwrap();
    let price = Decimal::from_f64(message.price).unwrap();
    let change = ((open - price) / open).round_dp(4).to_f32().unwrap();
    let timestamp = Utc::now().timestamp_millis();

    let ttl = match message.interval.as_str() {
      "1m" => 1440 * 60,
      "15m" => 672 * 900,
      "4h" => 126 * 14400,
      "1d" => 100 * 86400,
      _ => panic!("invalid interval {}", message.interval)
    };

    let redis_key = format!("{}:{}:{}:{}", Config::REDIS_KEY_KLINES, message.interval, message.symbol, message.timestamp);
    let _: () = rdb
      .hset_multiple(
        &redis_key[..],
        &[
          ("symbol", message.symbol),
          ("open", message.open.to_string()),
          ("price", message.price.to_string()),
          ("change", change.to_string()),
          ("high", message.high.to_string()),
          ("price", message.price.to_string()),
          ("low", message.low.to_string()),
          ("volume", message.volume.to_string()),
          ("quota", message.quota.to_string()),
          ("timestamp", timestamp.to_string()),
        ],
      )
      .await
      .unwrap();
      let _: () = rdb.expire(&redis_key[..], ttl).await.unwrap();
    Ok(())
  }

  pub async fn run(&self, ctx: &'a mut Ctx<'_>) -> Result<(), Box<dyn std::error::Error>> {
    if !["1m", "15m", "4h", "1d"].iter().any(|&s| s == self.interval) {
      return Err(Box::from("interval not valid"))
    }

    println!("streams tickres current {}", self.current);
    let mut symbols = self.scalping_repository.scan(ctx).expect("scalping scan failed");

    if self.current < 1 {
      return Err(Box::from("current less then 1"))
    }

    let size = Env::usize("BINANCE_SPOT_STREAMS_KLINES_SIZE".to_string());
    let offset = (usize::from(self.current) - 1) * size;
    if offset >= symbols.len() {
      return Err(Box::from("symbols out of range"))
    }

    if offset > 1 {
      let (_, items) = symbols.split_at(offset);
      symbols = items.to_vec();
    }

    if symbols.len() > size {
      let (items, _) = symbols.split_at(size);
      symbols = items.to_vec();
    }

    let endpoint = format!(
      "{}/stream?streams={}",
      Env::var("BINANCE_SPOT_STREAMS_ENDPOINT".to_string()),
      symbols.iter().map(
        |symbol| format!("{}@kline_{}", symbol.to_lowercase(), self.interval)
      ).collect::<Vec<_>>().join("/"),
    );
    println!("endpoint {endpoint:}");

    let rdb = Arc::new(tokio::sync::Mutex::new(ctx.rdb.clone()));

    let (stream, _) = connect_async(&endpoint).await.expect("Failed to connect");
    let (_, read) = stream.split();
    let read = Arc::new(tokio::sync::Mutex::new(read));
    println!("stream connected");
    let handle = tokio::spawn(async move {
      let mut read = read.lock_owned().await;
      while let Some(message) = read.next().await {
        let data = message.unwrap().into_data();
        //tokio::io::stdout().write(&data).await.unwrap();
        match serde_json::from_slice::<KlineEvent>(&data) {
          Ok(event) => {
            let mut rdb: tokio::sync::MutexGuard<'_, MultiplexedConnection> = rdb.lock().await;
            let _ = Self::process(&mut rdb, event.data.message).await;
          }
          Err(err) => println!("error: {}", err)
        }
      }
    });
    handle.await.expect("The read task failed.");

    Ok(())
  }
}