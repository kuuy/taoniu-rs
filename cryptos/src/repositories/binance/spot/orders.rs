use std::time::Duration;
use std::collections::HashMap;

use url::Url;
use base64::{engine::general_purpose, Engine as _};
use sha2::{Digest, Sha256};
use hmac::{Hmac, Mac};
use chrono::prelude::Utc;
use diesel::prelude::*;
use diesel::query_builder::QueryFragment;
use reqwest::header;
use rsa::{pkcs8::DecodePrivateKey, RsaPrivateKey};
use serde::{Deserialize, Deserializer};

use crate::common::*;
use crate::repositories::binance::ApiError;
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

#[derive(Deserialize)]
struct TradeInfo {
  symbol: String,
  #[serde(alias = "orderId")]
  order_id: i64,
  #[serde(alias = "type")]
  order_type: String,
  side: String,
  #[serde(deserialize_with = "to_f64")]
  price: f64,
  #[serde(alias = "origQty", deserialize_with = "to_f64")]
  quantity: f64,
  #[serde(alias = "executedQty", deserialize_with = "to_f64")]
  executed_quantity: f64,
  #[serde(alias = "transactTime")]
  transact_time: i64,
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
        Ok(result) => Ok(Some(result)),
        Err(diesel::result::Error::NotFound) => Ok(None),
        Err(err) => Err(err.into()),
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
      Err(err) => Err(err.into()),
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
      Err(err) => Err(err.into()),
    }
  }

  pub async fn submit<T>(
    ctx: Ctx,
    symbol: T,
    side: T,
    price: f64,
    quantity: f64,
  ) -> Result<i64, Box<dyn std::error::Error>> 
  where
    T: AsRef<str>
  {
    println!("orders submit");
    let symbol = symbol.as_ref();
    let side = side.as_ref();
    let price_val = price.to_string();
    let quantity_val = quantity.to_string();
    let timestamp = Utc::now().timestamp_millis().to_string();

    let mut params = HashMap::new();
    params.insert("symbol", symbol);
    params.insert("side", side);
    params.insert("type", "LIMIT");
    params.insert("price", &price_val);
    params.insert("quantity", &quantity_val);
    params.insert("timeInForce", "GTC");
    params.insert("newOrderRespType", "RESULT");
    params.insert("recvWindow", "60000");
    params.insert("timestamp", &timestamp);

    let payload = params.iter().map(|(k,v)| format!("{}={}", k, v)).collect::<Vec<_>>().join("&");
 
    let private_key = RsaPrivateKey::from_pkcs8_pem(&Env::var("BINANCE_SPOT_TRADE_API_SECRET"))?;
    let signature = private_key.sign(
      rsa::pkcs1v15::Pkcs1v15Sign::new::<rsa::sha2::Sha256>(),
      &Sha256::digest(payload.as_bytes()),
    )?;
    let signature = general_purpose::STANDARD.encode(signature);

    params.insert("signature", &signature);

    let url = Url::parse(format!("{}/api/v3/order", Env::var("BINANCE_SPOT_API_ENDPOINT")).as_str())?;

    let mut headers = header::HeaderMap::new();
    headers.insert("X-MBX-APIKEY", Env::var("BINANCE_SPOT_TRADE_API_KEY").parse().unwrap());

    let client = reqwest::Client::new();
    let response = client.post(url)
      .headers(headers)
      .form(&params)
      .timeout(Duration::from_secs(5))
      .send()
      .await?;

    let status_code = response.status();

    if status_code.is_server_error() {
      return Err(Box::new(ApiError{
        code: status_code.as_u16().into(),
        message: "".to_owned(),
      }))
    }

    if status_code.is_client_error() {
      match response.json::<ApiError>().await {
        Ok(err) => {
          return Err(Box::new(err))
        }
        Err(_) => return Err(Box::new(ApiError{
          code: status_code.as_u16().into(),
          message: "".to_owned(),
        }))
      }
    }

    if !status_code.is_success() {
      return Err(Box::from(format!("request error: {}", status_code)))
    }

    let trade = response.json::<TradeInfo>().await.unwrap();
    println!("response {:?}", trade.order_id);

    let id = xid::new().to_string();
    match Self::create(
      ctx.clone(),
      id,
      trade.symbol.to_owned(),
      trade.order_id,
      trade.order_type.to_owned(),
      trade.side.to_owned(),
      trade.price,
      0.0,
      trade.quantity,
      trade.executed_quantity,
      trade.transact_time,
      0,
      trade.status.clone(),
      "".to_owned(),
    ).await {
      Ok(result) => {
        println!("binance spot order {0:} {1:} create success {result:}", symbol, trade.order_id);
      }
      Err(err) => {
        println!("binance spot order {0:} {1:} create failed {err:?}", symbol, trade.order_id)
      }
    }

    Ok(trade.order_id)
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
      let entity: Option<Order> = match Self::get(ctx.clone(), order.symbol.clone(), order.order_id).await {
        Ok(Some(result)) => Some(result),
        Ok(None) => None,
        Err(err) => return Err(err.into()),
      };
      if entity.is_none() {
        let id = xid::new().to_string();
        match Self::create(
          ctx.clone(),
          id,
          symbol.to_owned(),
          order.order_id,
          order.order_type.to_owned(),
          order.side.to_owned(),
          order.price,
          order.stop_price,
          order.quantity,
          order.executed_quantity,
          order.open_time,
          order.update_time,
          order.status.to_owned(),
          "".to_string(),
        ).await {
          Ok(result) => {
            println!("binance spot order {0:} {1:} create success {result:}", order.symbol, order.order_id);
          }
          Err(err) => {
            println!("binance spot order {0:} {1:} create failed {err:?}", order.symbol, order.order_id)
          }
        }
      } else {
        let entity = entity.unwrap();
        if entity.price == order.price
          && entity.stop_price == order.stop_price
          && entity.quantity == order.quantity
          && entity.executed_quantity == order.executed_quantity
          && entity.update_time == order.update_time
          && entity.status == order.status {
          continue
        }
        match Self::update(
          ctx.clone(),
          entity.id,
          (
            orders::price.eq(order.price),
            orders::stop_price.eq(order.stop_price),
            orders::quantity.eq(order.quantity),
            orders::executed_quantity.eq(order.executed_quantity),
            orders::update_time.eq(order.update_time),
            orders::status.eq(order.status.to_owned()),
          ),
        ).await {
          Ok(result) => {
            println!("binance spot order {0:} {1:} update success {result:}", order.symbol, order.order_id);
          }
          Err(err) => {
            println!("binance spot order {0:} {1:} update failed {err:?}", order.symbol, order.order_id)
          }
        }
      }
    }

    Ok(())
  }
}
