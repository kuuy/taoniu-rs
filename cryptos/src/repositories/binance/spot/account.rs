use std::time::Duration;
use std::collections::HashMap;

use url::Url;
use sha2::Sha256;
use hmac::{Hmac, Mac};
use serde::{Deserialize, Deserializer};
use chrono::prelude::Utc;

use reqwest::header;

use crate::common::*;
use crate::config::binance::spot::config as Config;

pub struct AccountRepository {}

#[derive(Deserialize)]
struct AccountInfo {
  balances: Vec<Balance>,
}

#[derive(Deserialize)]
struct Balance {
  asset: String,
  #[serde(deserialize_with = "to_f64")]
  free: f64,
  #[serde(deserialize_with = "to_f64")]
  locked: f64,
}

fn to_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
  D: Deserializer<'de>,
{
  let s: &str = Deserialize::deserialize(deserializer)?;
  s.parse::<f64>().map_err(serde::de::Error::custom)
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

impl AccountRepository {
  pub async fn flush(ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    let timestamp = Utc::now().timestamp_millis().to_string();

    let mut params = HashMap::<&str, &str>::new();
    params.insert("recvWindow", "60000");
    params.insert("timestamp", &timestamp[..]);

    let mut url = Url::parse_with_params(format!("{}/api/v3/account", Env::var("BINANCE_SPOT_API_ENDPOINT")).as_str(), &params)?;
    let query: &str = match url.query() {
      Some(query) => query,
      None => "",
    };

    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_from_slice(Env::var("BINANCE_SPOT_ACCOUNT_API_SECRET").as_bytes()).unwrap();
    mac.update(query.as_bytes());
    let signature = hex::encode(&mac.finalize().into_bytes().to_vec());

    url.query_pairs_mut().append_pair("signature", signature.as_str());

    let mut headers = header::HeaderMap::new();
    headers.insert("X-MBX-APIKEY", Env::var("BINANCE_SPOT_ACCOUNT_API_KEY").parse().unwrap());

    let client = reqwest::Client::new();
    let response = client.get(url)
      .headers(headers)
      .timeout(Duration::from_secs(3))
      .send()
      .await?;

    let status_code = response.status();

    if status_code.is_client_error() {
      println!("response {}", response.text().await.unwrap());
      return Err(Box::from(format!("bad request: {}", status_code)))
    }

    if !status_code.is_success() {
      return Err(Box::from(format!("request error: {}", status_code)))
    }

    let account_info = response.json::<AccountInfo>().await.unwrap();
    println!("account_info {}", account_info.balances.len());

    let mut currencies: Vec<&str> = Vec::new();
    account_info.balances.iter().for_each(|coin| {
      if coin.free <= 0.0 {
        return;
      }
      currencies.push(&coin.asset[..]);
      println!("coin balance {} {} {}", coin.asset, coin.free, coin.locked);
    });

    println!("account flush");
    Ok(())
  }
}
