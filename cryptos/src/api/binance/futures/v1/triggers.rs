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
use crate::repositories::binance::futures::triggers::*;

#[derive(Deserialize)]
struct ListingsRequest {
  symbol: Option<String>,
  side: Option<i32>,
  current: u32,
  page_size: u32,
}

#[derive(Serialize)]
pub struct TriggerInfo {
  id: String,
  symbol: String,
  side: i32,
  capital: f64,
  price: f64,
  take_price: f64,
  stop_price: f64,
  take_order_id: i64,
  stop_order_id: i64,
  profit: f64,
  timestamp: i64,
  status: i32,
}

pub struct TriggersRouter {
  ctx: Ctx,
}

impl TriggersRouter {
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

    let side = match &request.side {
      Some(value) => *value,
      None => 0,
    };

    let current = request.current;
    if current < 1 {
      let message = ErrorMessage::new(false, "1004", "current not valid");
      return Err((StatusCode::FORBIDDEN, Json(serde_json::json!(message))))
    }

    let page_size = request.page_size;
    if page_size < 1 || page_size > 200 {
      let message = ErrorMessage::new(false, "1004", "page size not valid");
      return Err((StatusCode::FORBIDDEN, Json(serde_json::json!(message))))
    }

    let mut conditions = HashMap::<&str, MixValue>::new();
    if symbol != "" {
      conditions.insert("symbol", MixValue::String(symbol));
    }
    if side != 0 {
      conditions.insert("side", MixValue::Int(side));
    }

    let total = match TriggersRepository::count(ctx.clone(), &mut conditions).await {
      Ok(result) => result,
      Err(_) => {
        let message = ErrorMessage::new(false, "500", "database error");
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!(message))))
      },
    };
    let triggers = match TriggersRepository::listings(
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
      data: triggers.into_iter().map(|x: (String, String, i32, f64, f64, f64, f64, i64, i64, f64, i64, i32)| -> Box<dyn erased_serde::Serialize> { Box::new({
        let (id, symbol, side, capital, price, take_price, stop_price, take_order_id, stop_order_id, profit, timestamp, status) = x;
        TriggerInfo{
          id,
          symbol,
          side,
          capital,
          price,
          take_price,
          stop_price,
          take_order_id,
          stop_order_id,
          profit,
          timestamp,
          status,
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
