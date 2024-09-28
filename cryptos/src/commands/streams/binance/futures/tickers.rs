use std::sync::Arc;
use futures_util::stream::StreamExt;

use chrono::prelude::Utc;
use rust_decimal::prelude::*;
use redis::AsyncCommands;
use serde::{Deserialize, Deserializer};
use tokio_tungstenite::{tungstenite::Message, connect_async};
use clap::{Parser};

use crate::common::*;
use crate::config::binance::futures::config as Config;
use crate::repositories::binance::futures::scalping::*;

#[derive(Parser)]
pub struct TickersCommand {
  #[clap(default_value_t = 1)]
  current: u8,
}

impl Default for TickersCommand {
  fn default() -> Self {
    Self::new()
  }
}

#[derive(Deserialize)]
struct TickerEvent {
  #[serde(alias = "data")]
  message: TickerMessage,
}

#[derive(Deserialize)]
struct TickerMessage {
  #[serde(alias = "s")]
  symbol: String,
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
  #[serde(alias = "E")]
  timestamp: i64,
}

fn to_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
  D: Deserializer<'de>,
{
  let s: &str = Deserialize::deserialize(deserializer)?;
  s.parse::<f64>().map_err(serde::de::Error::custom)
}

impl TickersCommand {
  pub fn new() -> Self {
    Self {
      ..Default::default()
    }
  }

  async fn process(ctx: Ctx, message: TickerMessage) {
    println!("process message {} {}", message.symbol, message.timestamp);
    let open = Decimal::from_f64(message.open).unwrap();
    let price = Decimal::from_f64(message.price).unwrap();
    let change = ((open - price) / open).round_dp(4).to_f32().unwrap();
    let timestamp = Utc::now().timestamp_millis();

    let mut rdb = ctx.rdb.lock().await.clone();
    rdb.hset_multiple(
      format!("{}:{}", Config::REDIS_KEY_TICKERS, message.symbol), 
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
    ).await.unwrap()
  }

  pub async fn run(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    println!("streams tickres current {}", self.current);
    let mut symbols = ScalpingRepository::scan(ctx.clone()).await.unwrap();

    if self.current < 1 {
      return Err(Box::from("current less then 1"))
    }

    let size = Env::usize("BINANCE_FUTURES_STREAMS_TICKERS_SIZE");
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
      Env::var("BINANCE_FUTURES_STREAMS_ENDPOINT"),
      symbols.iter().map(
        |symbol| format!("{}@miniTicker", symbol.to_lowercase())
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
          match message.unwrap() {
            Message::Text(content) => {
              match serde_json::from_str::<TickerEvent>(&content) {
                Ok(event) => {
                  let _ = Self::process(ctx.clone(), event.message).await;
                }
                Err(err) => println!("error: {}", err)
              }
            },
            _ => break,
          }
        }
      }
    }));
    handle.await.expect("The read task failed.");

    Ok(())
  }
}
