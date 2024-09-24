use std::time::Duration;
use std::sync::Arc;
use futures_util::stream::StreamExt;

use chrono::prelude::Utc;
use rust_decimal::prelude::*;
use redis::AsyncCommands;
use serde::{Deserialize, Deserializer};
use tokio_tungstenite::connect_async;
use clap::{Parser};

use crate::common::*;
use crate::config::binance::spot::config as Config;
use crate::repositories::binance::spot::scalping::*;

#[derive(Parser)]
pub struct KlinesCommand {
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
  close: f64,
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

impl KlinesCommand {
  pub fn new() -> Self {
    Self {
      ..Default::default()
    }
  }

  async fn process(ctx: Ctx, message: KlineMessage) -> Result<(), Box<dyn std::error::Error>> {
    println!("process message {} {}", message.symbol, message.timestamp);
    let open = Decimal::from_f64(message.open).unwrap();
    let close = Decimal::from_f64(message.close).unwrap();
    let change = ((open - close) / open).round_dp(4).to_f32().unwrap();
    let timestamp = Utc::now().timestamp_millis();

    let ttl: Duration = match message.interval.as_str() {
      "1m" => Duration::from_secs(30+60),
      "15m" => Duration::from_secs(30+900),
      "4h" => Duration::from_secs(30+14400),
      "1d" => Duration::from_secs(30+86400),
      _ => panic!("invalid interval {}", message.interval)
    };

    let mut rdb = ctx.rdb.lock().await.clone();
    let redis_key = format!("{}:{}:{}:{}", Config::REDIS_KEY_KLINES, message.interval, message.symbol, message.timestamp);
    let is_exists: bool = rdb.exists(&redis_key[..]).await.unwrap();
    rdb.hset_multiple(
      &redis_key[..],
      &[
        ("symbol", message.symbol),
        ("open", message.open.to_string()),
        ("close", message.close.to_string()),
        ("change", change.to_string()),
        ("high", message.high.to_string()),
        ("low", message.low.to_string()),
        ("volume", message.volume.to_string()),
        ("quota", message.quota.to_string()),
        ("timestamp", timestamp.to_string()),
      ],
    ).await?;
    if !is_exists {
      rdb.expire(&redis_key[..], ttl.as_secs().try_into().unwrap()).await?;
    }
    Ok(())
  }

  pub async fn run(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    if !["1m", "15m", "4h", "1d"].iter().any(|&s| s == self.interval) {
      return Err(Box::from("interval not valid"))
    }

    println!("streams tickres current {}", self.current);
    let mut symbols = ScalpingRepository::scan(ctx.clone()).await.unwrap();

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

    let (stream, _) = connect_async(&endpoint).await.expect("Failed to connect");
    let (_, read) = stream.split();
    let read = Arc::new(tokio::sync::Mutex::new(read));
    println!("stream connected");
    let handle = tokio::spawn(Box::pin({
      let ctx = ctx.clone();
      let mut read = read.lock_owned().await;
      async move {
        while let Some(message) = read.next().await {
          let data = message.unwrap().into_data();
          // tokio::io::stdout().write(&data).await.unwrap();
          match serde_json::from_slice::<KlineEvent>(&data) {
            Ok(event) => {
              let _ = Self::process(ctx.clone(), event.data.message).await;
            }
            Err(err) => println!("error: {}", err)
          }
        }
      }
    }));
    handle.await.expect("The read task failed.");

    Ok(())
  }
}
