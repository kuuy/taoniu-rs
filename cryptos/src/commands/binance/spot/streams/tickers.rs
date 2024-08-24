use rust_decimal::prelude::*;
use redis::aio::MultiplexedConnection;
use futures_util::stream::StreamExt;
use serde::{Deserialize, Deserializer};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use clap::{Parser, Subcommand};

use crate::common::*;
use crate::config::binance::spot::config as Config;
use crate::repositories::SymbolsRepository;

#[derive(Parser)]
pub struct TickersCommand {
  #[clap(skip)]
  symbolsRepository: SymbolsRepository,
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
  #[serde(alias = "o", deserialize_with = "to_f32")]
  open: f32,
  #[serde(alias = "c", deserialize_with = "to_f32")]
  price: f32,
  #[serde(skip_deserializing)]
  change: f32,
  #[serde(alias = "h", deserialize_with = "to_f32")]
  high: f32,
  #[serde(alias = "l", deserialize_with = "to_f32")]
  low: f32,
  #[serde(alias = "v", deserialize_with = "to_f64")]
  volume: f64,
  #[serde(alias = "q", deserialize_with = "to_f64")]
  quota: f64,
}

fn to_f32<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
  D: Deserializer<'de>,
{
  let s: &str = Deserialize::deserialize(deserializer)?;
  s.parse::<f32>().map_err(serde::de::Error::custom)
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
      symbolsRepository: SymbolsRepository{},
      ..Default::default()
    }
  }

  pub async fn run(&self, ctx: &'a mut Ctx<'_>) -> Result<(), Box<dyn std::error::Error>> {
    println!("streams tickres");
    let mut symbols: Vec<String> = Vec::new();
    symbols.push("btcusdt".to_string());
    symbols.push("ethusdt".to_string());

    let endpoint = format!(
      "{}/stream?streams={}",
      Env::var("BINANCE_SPOT_STREAMS_ENDPOINT".to_string()),
      symbols.iter().map(
        |symbol| format!("{symbol:}@miniTicker")
      ).collect::<Vec<_>>().join("/"),
    );
    println!("endpoint {endpoint:}");

    let (stream, _) = connect_async(&endpoint).await.expect("Failed to connect");
    let (_,mut read) = stream.split();
    println!("stream connected");
    let handle = tokio::spawn(async move {
      while let Some(message) = read.next().await {
        let data = message.unwrap().into_data();
        let mut event: TickerEvent = serde_json::from_slice(&data).expect("message parse failed");
        let open = Decimal::from_f32(event.message.open).unwrap();
        let price = Decimal::from_f32(event.message.price).unwrap();
        event.message.change = ((open - price) / open).round_dp(4).to_f32().unwrap();
        println!(
          "ticker message {} open:{} price:{} change:{}, high:{} low:{} volume:{} quota:{}",
          event.message.symbol,
          event.message.open,
          event.message.price,
          event.message.change,
          event.message.high,
          event.message.low,
          event.message.volume,
          event.message.quota,
        );
      }
    });
    handle.await.expect("The read task failed.");

    Ok(())
  }
}
