use std::time::Duration;
use std::collections::HashMap;

use url::Url;
use sha2::{Digest, Sha256};
use hmac::{Hmac, Mac};
use chrono::{prelude::Utc, Timelike};
use diesel::prelude::*;
use diesel::query_builder::QueryFragment;
use diesel::ExpressionMethods;
use reqwest::header;
use rsa::{pkcs8::DecodePrivateKey, RsaPrivateKey};
use serde::{Deserialize, Deserializer};

use crate::common::*;
use crate::schema::binance::spot::orders::*;
use crate::models::binance::spot::order::*;

#[derive(Deserialize)]
struct OrderInfo {
  symbol: String,
  #[serde(alias = "orderId")]
  order_id: i64,
  #[serde(alias = "type")]
  order_type: String,
  side: String,
  #[serde(deserialize_with = "to_f64")]
  price: f64,
  #[serde(alias = "stopPrice", deserialize_with = "to_f64")]
  stop_price: f64,
  #[serde(alias = "origQty", deserialize_with = "to_f64")]
  quantity: f64,
  #[serde(alias = "executedQty", deserialize_with = "to_f64")]
  executed_quantity: f64,
  #[serde(alias = "time")]
  open_time: i64,
  #[serde(alias = "updateTime")]
  update_time: i64,
  status: String,
}

fn to_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
  D: Deserializer<'de>,
{
  let s: &str = Deserialize::deserialize(deserializer)?;
  s.parse::<f64>().map_err(serde::de::Error::custom)
}

pub struct OrdersRepository {}

impl OrdersRepository {
  pub async fn get<T>(
    ctx: Ctx,
    symbol: T,
    order_id: i64,
  ) -> Result<Option<Order>, Box<dyn std::error::Error>>
  where
    T: AsRef<str>
  {
    let symbol = symbol.as_ref();

    let pool = ctx.pool.read().await;
    let mut conn = pool.get().unwrap();

    match orders::table
      .select(Order::as_select())
      .filter(orders::symbol.eq(symbol))
      .filter(orders::order_id.eq(order_id))
      .first(&mut conn) {
      Ok(order) => Ok(Some(order)),
      Err(diesel::result::Error::NotFound) => Ok(None),
      Err(e) => Err(e.into()),
    }
  }

  pub async fn create(
    ctx: Ctx,
    id: String,
    symbol: String,
    order_id: i64,
    order_type: String,
    side: String,
    price: f64,
    stop_price: f64,
    quantity: f64,
    executed_quantity: f64,
    open_time: i64,
    update_time: i64,
    status: String,
    remark: String,
  ) -> Result<bool, Box<dyn std::error::Error>> {
    let pool = ctx.pool.write().await;
    let mut conn = pool.get().unwrap();

    let now = Utc::now();
    let order = Order::new(
      id,
      symbol,
      order_id,
      order_type,
      side,
      price,
      0.0,
      stop_price,
      quantity,
      executed_quantity,
      open_time,
      update_time,
      status,
      remark,
      now,
      now,
    );
    match diesel::insert_into(orders::table)
      .values(&order)
      .execute(&mut conn) {
      Ok(effective_rows) => Ok(effective_rows > 0),
      Err(e) => Err(e.into()),
    }
  }

  pub async fn update<V>(
    ctx: Ctx,
    id: String,
    value: V,
  ) -> Result<bool, Box<dyn std::error::Error>> 
  where
    V: diesel::AsChangeset<Target = orders::table>,
    <V as diesel::AsChangeset>::Changeset: QueryFragment<diesel::pg::Pg>,
  {
    let pool = ctx.pool.write().await;
    let mut conn = pool.get().unwrap();
    match diesel::update(orders::table.find(id)).set(value).execute(&mut conn) {
      Ok(effective_rows) => Ok(effective_rows > 0),
      Err(e) => Err(e.into()),
    }
  }

  pub async fn open(ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    println!("orders open");
    let payload = "symbol=BTCUSDT&timestamp=";

    let private_key = RsaPrivateKey::from_pkcs8_pem(&Env::var("BINANCE_SPOT_TRADE_API_SECRET"))?;
    let signature = private_key.sign(
      rsa::pkcs1v15::Pkcs1v15Sign::new::<rsa::sha2::Sha256>(),
      &Sha256::digest(payload.as_bytes()),
    )?;
    println!("signature {:?}", base64::encode(signature));
    if 1 > 0 {
      return Err(Box::from("orders repository open failed"))
    }
    Ok(())
  }

  pub async fn sync<T>(
    ctx: Ctx,
    symbol: T,
    start_time: i64,
    limit: i64,
  ) -> Result<(), Box<dyn std::error::Error>> 
  where
    T: AsRef<str>
  {
    let symbol = symbol.as_ref();
    let start_time_val = start_time.to_string();
    let limit = limit.to_string();
    let timestamp = Utc::now().timestamp_millis().to_string();

    let mut params = HashMap::new();
    params.insert("symbol", symbol);
    if start_time > 0 {
      params.insert("startTime", &start_time_val);
    }
    params.insert("limit", &limit);
    params.insert("recvWindow", "60000");
    params.insert("timestamp", &timestamp);

    let mut url = Url::parse_with_params(format!("{}/api/v3/allOrders", Env::var("BINANCE_SPOT_API_ENDPOINT")).as_str(), &params)?;
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

    let orders = response.json::<Vec<OrderInfo>>().await.unwrap();

    for order in orders.iter() {
      let mut entity: Option<Order> = None;
      match Self::get(ctx.clone(), order.symbol.clone(), order.order_id).await {
        Ok(Some(result)) => {
          entity = Some(result);
        },
        Ok(None) => {},
        Err(e) => return Err(e.into()),
      }
      let mut success = false;
      if entity.is_none() {
        let id = xid::new().to_string();
        match Self::create(
          ctx.clone(), 
          id,
          symbol.to_string(),
          order.order_id,
          order.order_type.clone(),
          order.side.clone(),
          order.price,
          order.stop_price,
          order.quantity,
          order.executed_quantity,
          order.open_time,
          order.update_time,
          order.status.clone(),
          "".to_string(),
        ).await {
          Ok(result) => {
            if result {
              success = result;
            }
            println!("binance spot order {0:} {1:} create success {result:}", order.symbol, order.order_id);
          }
          Err(e) => {
            println!("binance spot order {0:} {1:} create failed {e:?}", order.symbol, order.order_id)
          },
        }
      } else {
        let entity = entity.unwrap();
        match Self::update(
          ctx.clone(),
          entity.id,
          (
            orders::executed_quantity.eq(order.executed_quantity.clone()),
            orders::update_time.eq(order.update_time.clone()),
            orders::status.eq(order.status.clone()),
          ),
        ).await {
          Ok(result) => {
            success = result;
            println!("binance spot order {0:} {1:} update success {result:}", order.symbol, order.order_id);
          }
          Err(e) => {
            println!("binance spot order {0:} {1:} update failed {e:?}", order.symbol, order.order_id)
          },
        }
      }
    }

    Ok(())
  }
}
