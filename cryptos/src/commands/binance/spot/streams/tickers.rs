use std::time::Duration;
use std::sync::Arc;

use chrono::prelude::Utc;
use rust_decimal::prelude::*;
use redis::AsyncCommands;
use redis::aio::MultiplexedConnection;
use futures_util::stream::StreamExt;
use serde::{Deserialize, Deserializer};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::{connect_async, tungstenite, tungstenite::protocol::Message};
use clap::{Args};

use crate::common::*;
use crate::config::binance::spot::config as Config;
use crate::repositories::ScalpingRepository;

#[derive(Args)]
pub struct TickersCommand {
  #[clap(skip)]
  scalpingRepository: ScalpingRepository,
  #[clap(default_value_t = 0)]
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

impl<'a> TickersCommand {
  pub fn new() -> Self {
    Self {
      scalpingRepository: ScalpingRepository{},
      ..Default::default()
    }
  }

  async fn process(rdb: &mut tokio::sync::MutexGuard<'_, MultiplexedConnection>, message: TickerMessage) {
    println!("process message {} {}", message.symbol, message.timestamp);
    let open = Decimal::from_f64(message.open).unwrap();
    let price = Decimal::from_f64(message.price).unwrap();
    let change = ((open - price) / open).round_dp(4).to_f32().unwrap();
    let timestamp = Utc::now().timestamp_millis();

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

  pub async fn run(&self, ctx: &'a mut Ctx<'_>) -> Result<(), Box<dyn std::error::Error>> {
    println!("streams tickres current {}", self.current);
    let symbols = self.scalpingRepository.scan(ctx).expect("scalping scan failed");

    let len = symbols.len();
    let offset = (usize::from(self.current) - 1) * 33;
    if offset >= len {
      return Err(Box::from("symbols out of range"))
    }
    let endpos = (offset + 33).min(len);

    println!("symbols split {} {} {} {}", len, usize::from(self.current) - 1, offset, endpos);
  
    let mut vars: Vec<String> = Vec::new();
    if offset > 1 {
      let (_, items) = symbols.split_at(offset);
      vars = items.to_vec();
    } else {
      vars = symbols.clone();
    }

    println!("symbols split {} {}", vars.len(), endpos-offset);
    let (items, _) = vars.split_at(endpos-offset);
    vars = items.to_vec();

    let endpoint = format!(
      "{}/stream?streams={}",
      Env::var("BINANCE_SPOT_STREAMS_ENDPOINT".to_string()),
      symbols[offset..endpos].iter().map(
        |symbol| format!("{}@miniTicker", symbol.to_lowercase())
      ).collect::<Vec<_>>().join("/"),
    );
    println!("endpoint {endpoint:}");

    let rdb = Arc::new(tokio::sync::Mutex::new(ctx.rdb.clone()));

    let (stream, _) = connect_async(&endpoint).await.expect("Failed to connect");
    let (_,mut read) = stream.split();
    let read = Arc::new(tokio::sync::Mutex::new(read));
    println!("stream connected");
    let handle = tokio::spawn(async move {
      let mut read = read.lock_owned().await;
      while let Some(message) = read.next().await {
        let data = message.unwrap().into_data();
        // tokio::io::stdout().write(&data).await.unwrap();
        match serde_json::from_slice::<TickerEvent>(&data) {
          Ok(event) => {
            let mut rdb: tokio::sync::MutexGuard<'_, MultiplexedConnection> = rdb.lock().await;
            Self::process(&mut rdb, event.message).await;
          }
          Err(err) => {
            println!("error: {}", err)
          }
          _ => {}
        }
      }
    });
    handle.await.expect("The read task failed.");

    Ok(())
  }
}
