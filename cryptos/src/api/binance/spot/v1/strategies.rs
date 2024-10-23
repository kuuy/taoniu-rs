use std::collections::HashMap;

use axum::{
  extract::{State, Query},
  routing::get,
  http::StatusCode,
  Json,
  Router,
};
use serde::{Deserialize, Serialize};

use crate::common::*;
use crate::api::response::*;
use crate::repositories::binance::spot::strategies::*;

#[derive(Deserialize)]
struct ListingsRequest {
  symbol: Option<String>,
  signal: Option<String>,
  current: u32,
  page_size: u32,
}

#[derive(Serialize)]
pub struct StrategiesInfo {
  id: String,
  symbol: String,
  indicator: String,
  signal: i32,
  price: f64,
  timestamp: i64,
}

pub struct StrategiesRouter {
  ctx: Ctx,
}

impl StrategiesRouter {
  pub fn new(ctx: Ctx) -> Self {
    Self {
      ctx: ctx,
    }
  }

  async fn listings(
    State(ctx): State<Ctx>,
    request: Query<ListingsRequest>,
  ) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let symbol = match &request.symbol {
      Some(value) => value.clone(),
      None => "".to_owned(),
    };
    let signal = match &request.signal {
      Some(value) => value.clone(),
      None => "".to_owned(),
    };

    let current = request.current;
    if current < 1 {
      let message = ErrorMessage::new(false, "1004", "current not valid");
      return Err((StatusCode::FORBIDDEN, Json(serde_json::json!(message))))
    }

    let page_size = request.page_size;
    if page_size < 1 || page_size > 100 {
      let message = ErrorMessage::new(false, "1004", "page size not valid");
      return Err((StatusCode::FORBIDDEN, Json(serde_json::json!(message))))
    }

    let mut conditions = HashMap::<&str, MixValue>::new();
    if symbol != "" {
      conditions.insert("symbol", MixValue::String(symbol));
    }
    if signal != "" {
      let signal = signal.parse::<i32>().unwrap_or(0);
      conditions.insert("signal", MixValue::Int(signal));
    }

    let total = match StrategiesRepository::count(ctx.clone(), &mut conditions).await {
      Ok(result) => result,
      Err(_) => {
        let message = ErrorMessage::new(false, "500", "database error");
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!(message))))
      },
    };
    let strategies = match StrategiesRepository::listings(
      ctx.clone(),
      &mut conditions,
      current.into(),
      page_size.into(),
    ).await {
      Ok(result) => result,
      Err(_) => {
        let message = ErrorMessage::new(false, "500", "database error");
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!(message))))
      }
    };

    let response = PagenateResponse{
      success: true,
      data: strategies.into_iter().map(|x: (String, String, String, i32, f64, i64)| -> Box<dyn erased_serde::Serialize> { Box::new({
        let (id, symbol, indicator, signal, price, timestamp) = x;
        StrategiesInfo{
          id,
          symbol,
          indicator,
          signal,
          price,
          timestamp,
        }
      }) }).collect(),
      total,
      current,
      page_size,
    };
    Ok(Json(serde_json::json!(response)))
  }

  pub fn routes(&self) -> Router {
    let ctx = self.ctx.clone();
    return Router::new()
      .route("/", get(Self::listings))
      .with_state(ctx);
  }
}
