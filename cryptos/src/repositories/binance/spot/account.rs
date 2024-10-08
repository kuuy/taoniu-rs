use std::time::Duration;
use std::collections::HashMap;

use url::Url;
use sha2::Sha256;
use hmac::{Hmac, Mac};
use serde::{Deserialize, Deserializer};
use chrono::prelude::Utc;
use reqwest::header;
use redis::AsyncCommands;

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

impl AccountRepository {
  pub async fn balance<T>(ctx: Ctx, asset: T) -> Result<(f64, f64), Box<dyn std::error::Error>> 
  where
    T: AsRef<str>
  {
    let asset = asset.as_ref();

    let mut rdb = ctx.rdb.lock().await.clone();

    let redis_key = format!("{}:{}", Config::REDIS_KEY_BALANCE, asset);
    let fields = vec!["free", "locked"];
    match redis::cmd("HMGET")
      .arg(&redis_key)
      .arg(&fields)
      .query_async(&mut rdb)
      .await
    {
      Ok((Some(free), Some(locked))) => Ok((free, locked)),
      Ok(_) => return Err(Box::from(format!("balance of {asset:} not exists"))),
      Err(err) => return Err(err.into()),
    }
  }

  pub async fn flush(ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    let timestamp = Utc::now().timestamp_millis().to_string();

    let mut params = HashMap::<&str, &str>::new();
    params.insert("recvWindow", "60000");
    params.insert("timestamp", &timestamp);

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

    let mut rdb = ctx.rdb.lock().await.clone();

    let mut currencies: Vec<String> = Vec::new();
    let last_currencies: Vec<String> = rdb.smembers(Config::REDIS_KEY_CURRENCIES).await.unwrap();

    let mut pipe = redis::pipe();
    account_info.balances.iter().for_each(|coin| {
      if coin.free <= 0.0 {
        return;
      }
      pipe.hset_multiple(
        format!("{}:{}", Config::REDIS_KEY_BALANCE, coin.asset), 
        &[
          ("free", coin.free.to_string()),
          ("locked", coin.locked.to_string()),
        ],
      );
      pipe.sadd(Config::REDIS_KEY_CURRENCIES, &coin.asset);
      currencies.push(coin.asset.clone());
      println!("coin balance {} {} {}", coin.asset, coin.free, coin.locked);
    });

    last_currencies.iter().for_each(|last_asset| {
      if currencies.iter().any(|asset| asset == last_asset) {
        return;
      }
      pipe.srem(Config::REDIS_KEY_CURRENCIES, &last_asset);
      pipe.del(format!("{}:{}", Config::REDIS_KEY_BALANCE, last_asset));
      println!("coin balance remove {}", last_asset);
    });

    pipe.query_async(&mut rdb).await?;

    println!("account flush");
    Ok(())
  }
}
