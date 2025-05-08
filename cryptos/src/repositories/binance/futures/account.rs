use std::time::Duration;
use std::collections::HashMap;

use url::Url;
use sha2::Sha256;
use hmac::{Hmac, Mac};
use reqwest::header;

use chrono::prelude::Utc;
use diesel::prelude::*;
use redis::AsyncCommands;
use serde::{Deserialize, Deserializer};

use crate::common::*;
use crate::repositories::binance::futures::positions::*;
use crate::schema::binance::futures::positions::*;
use crate::config::binance::futures::config as Config;

pub struct AccountRepository {}

#[derive(Deserialize)]
struct AccountInfo {
  assets: Vec<Balance>,
  positions: Vec<PositionInfo>,
}

#[derive(Debug, Deserialize)]
struct Balance {
  asset: String,
  #[serde(alias = "walletBalance", deserialize_with = "to_f64")]
  balance: f64,
  #[serde(alias = "availableBalance", deserialize_with = "to_f64")]
  free: f64,
  #[serde(alias = "unrealizedProfit", deserialize_with = "to_f64")]
  unrealized_profit: f64,
  #[serde(alias = "marginBalance", deserialize_with = "to_f64")]
  margin: f64,
  #[serde(alias = "initialMargin", deserialize_with = "to_f64")]
  initial_margin: f64,
  #[serde(alias = "maintMargin", deserialize_with = "to_f64")]
  maint_margin: f64,
}

#[derive(Debug, Deserialize)]
struct PositionInfo {
  symbol: String,
  #[serde(alias = "positionSide")]
  position_side: String,
  isolated: bool,
  #[serde(deserialize_with = "to_i32")]
  leverage: i32,
  #[serde(alias = "maxNotional", deserialize_with = "to_f64")]
  capital: f64,
  #[serde(deserialize_with = "to_f64")]
  notional: f64,
  #[serde(alias = "entryPrice", deserialize_with = "to_f64")]
  entry_price: f64,
  #[serde(alias = "positionAmt", deserialize_with = "to_f64")]
  entry_quantity: f64,
  #[serde(alias = "updateTime")]
  update_time: i64,
}

fn to_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
  D: Deserializer<'de>,
{
  let s: &str = Deserialize::deserialize(deserializer)?;
  s.parse::<f64>().map_err(serde::de::Error::custom)
}

fn to_i32<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
  D: Deserializer<'de>,
{
  let s: &str = Deserialize::deserialize(deserializer)?;
  s.parse::<i32>().map_err(serde::de::Error::custom)
}

impl AccountRepository {
  pub async fn balance<T>(ctx: Ctx, asset: T) -> Result<(f64, f64, f64, f64, f64, f64), Box<dyn std::error::Error>> 
  where
    T: AsRef<str>
  {
    let asset = asset.as_ref();

    let mut rdb = ctx.rdb.lock().await.clone();

    let redis_key = format!("{}:{}", Config::REDIS_KEY_BALANCE, asset);
    let fields = vec!["balance", "free", "unrealized_profit", "margin", "initial_margin", "maint_margin"];
    match redis::cmd("HMGET")
      .arg(&redis_key)
      .arg(&fields)
      .query_async(&mut rdb)
      .await
    {
      Ok((
        Some(balance),
        Some(free),
        Some(unrealized_profit),
        Some(margin),
        Some(initial_margin),
        Some(maint_margin)),
      ) => Ok((
        balance,
        free,
        unrealized_profit,
        margin,
        initial_margin,
        maint_margin,
      )),
      Ok(_) => return Err(Box::from(format!("balance of {asset:} not exists"))),
      Err(err) => return Err(err.into()),
    }
  }

  pub async fn flush(ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    let timestamp = Utc::now().timestamp_millis().to_string();

    let mut params = HashMap::<&str, &str>::new();
    params.insert("timeInForce", "GTC");
    params.insert("recvWindow", "60000");
    params.insert("timestamp", &timestamp);

    let mut url = Url::parse_with_params(format!("{}/fapi/v2/account", Env::var("BINANCE_FUTURES_API_ENDPOINT")).as_str(), &params)?;
    let query: &str = match url.query() {
      Some(query) => query,
      None => "",
    };

    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_from_slice(Env::var("BINANCE_FUTURES_ACCOUNT_API_SECRET").as_bytes()).unwrap();
    mac.update(query.as_bytes());
    let signature = hex::encode(&mac.finalize().into_bytes().to_vec());

    url.query_pairs_mut().append_pair("signature", signature.as_str());

    let mut headers = header::HeaderMap::new();
    headers.insert("X-MBX-APIKEY", Env::var("BINANCE_FUTURES_ACCOUNT_API_KEY").parse().unwrap());

    let client = reqwest::Client::new();
    let response = client.get(url)
      .headers(headers)
      .timeout(Duration::from_secs(30))
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

    let mut rdb = ctx.rdb.lock().await.clone();

    let mut currencies: Vec<String> = Vec::new();
    let last_currencies: Vec<String> = rdb.smembers(Config::REDIS_KEY_CURRENCIES).await.unwrap();

    let mut pipe = redis::pipe();
    account_info.assets.iter().for_each(|coin| {
      if coin.free <= 0.0 {
        return;
      }
      pipe.hset_multiple(
        format!("{}:{}", Config::REDIS_KEY_BALANCE, coin.asset),
        &[
          ("balance", coin.balance.to_string()),
          ("free", coin.free.to_string()),
          ("unrealized_profit", coin.unrealized_profit.to_string()),
          ("margin", coin.margin.to_string()),
          ("initial_margin", coin.initial_margin.to_string()),
          ("maint_margin", coin.maint_margin.to_string()),
        ],
      );
      pipe.sadd(Config::REDIS_KEY_CURRENCIES, &coin.asset);
      currencies.push(coin.asset.clone());
      println!("coin balance {} {} {}", coin.asset, coin.balance, coin.unrealized_profit);
    });

    last_currencies.iter().for_each(|last_asset| {
      if currencies.iter().any(|asset| asset == last_asset) {
        return;
      }
      pipe.srem(Config::REDIS_KEY_CURRENCIES, &last_asset);
      pipe.del(format!("{}:{}", Config::REDIS_KEY_BALANCE, last_asset));
      println!("coin balance remove {}", last_asset);
    });

    () = pipe.query_async(&mut rdb).await?;

    for position in account_info.positions.iter() {
      if position.isolated || position.update_time == 0 {
        continue
      }

      if position.position_side != "LONG" && position.position_side != "SHORT" {
        continue
      }

      let mut side: i32 = 1;
      if position.position_side == "SHORT" {
        side = 2;
      }
      let mut entry_quantity = position.entry_quantity;

      if side == 1 && entry_quantity < 0.0 {
        entry_quantity = 0.0;
      }
      if side == 2 && entry_quantity > 0.0 {
        entry_quantity = 0.0;
      }
      if entry_quantity < 0.0 {
        entry_quantity = -entry_quantity;
      }

      let entity = match PositionsRepository::get(ctx.clone(), position.symbol.clone(), side).await {
        Ok(Some(result)) => Some(result),
        Ok(None) => None,
        Err(err) => return Err(err.into()),
      };
      if entity.is_none() {
        if entry_quantity == 0.0 {
          continue
        }
        let id = xid::new().to_string();
        match PositionsRepository::create(
          ctx.clone(),
          id,
          position.symbol.clone(),
          side,
          position.leverage,
          position.capital,
          position.notional,
          position.entry_price,
          entry_quantity,
          Utc::now().timestamp_micros(),
          1,
        ).await {
          Ok(result) => {
            println!("binance futures position {0:} {side:} create success {result:}", position.symbol);
          }
          Err(err) => {
            println!("binance futures position {0:} {side:} create failed {err:?}", position.symbol)
          }
        }
      } else {
        let entity = entity.unwrap();
        if entity.entry_price == position.entry_price && entity.entry_quantity == entry_quantity {
          continue
        }
        match PositionsRepository::update(
          ctx.clone(),
          entity.id,
          entity.version,
          (
            positions::leverage.eq(position.leverage),
            positions::capital.eq(position.capital),
            positions::notional.eq(position.notional),
            positions::entry_price.eq(position.entry_price),
            positions::entry_quantity.eq(entry_quantity),
            positions::timestamp.eq(Utc::now().timestamp_micros()),
            positions::version.eq(positions::version + 1),
          ),
        ).await {
          Ok(result) => {
            println!("binance futures kline {0:} {side:} update success {result:}", position.symbol);
          }
          Err(err) => {
            println!("binance futures kline {0:} {side:} update failed {err:?}", position.symbol)
          }
        }
      }
    }

    Ok(())
  }
}
