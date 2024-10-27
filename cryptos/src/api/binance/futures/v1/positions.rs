use std::collections::HashMap;

use axum::{
  extract::{State, Query},
  routing::get,
  http::StatusCode,
  Json,
  Router,
};
use serde::{Serialize, Deserialize};

use crate::common::*;
use crate::api::response::*;
use crate::repositories::binance::futures::positions::*;

#[derive(Deserialize)]
struct GetsRequest {
  side: Option<String>,
}

pub struct PositionsRouter {
  ctx: Ctx,
}

#[derive(Serialize)]
pub struct PositionInfo {
  id: String,
  symbol: String,
  side: i32,
  leverage: i32,
  capital: f64,
  notional: f64,
  entry_price: f64,
  entry_quantity: f64,
  entry_amount: f64,
  timestamp: i64,
}

impl PositionsRouter {
  pub fn new(ctx: Ctx) -> Self {
    Self {
      ctx: ctx,
    }
  }

  async fn gets(
    State(ctx): State<Ctx>,
    request: Query<GetsRequest>,
  ) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let side = match &request.side {
      Some(value) => value.clone(),
      None => "".to_owned(),
    };

    let mut conditions = HashMap::<&str, MixValue>::new();
    if side != "" {
      let side = side.parse::<i32>().unwrap_or(0);
      conditions.insert("side", MixValue::Int(side));
    }

    let positions = match PositionsRepository::gets(ctx.clone(), &mut conditions).await {
      Ok(result) => result,
      Err(_) => {
        let message = ErrorMessage::new(false, "500", "database error");
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!(message))))
      }
    };
    let response = RankingResponse{
      success: true,
      data: positions.into_iter().map(|x: (String, String, i32, i32, f64, f64, f64, f64, i64)| -> Box<dyn erased_serde::Serialize> { Box::new({
        let (id, symbol, side, leverage, capital, notional, entry_price, entry_quantity, timestamp) = x;
        PositionInfo{
          id,
          symbol,
          side,
          leverage,
          capital,
          notional,
          entry_price,
          entry_quantity,
          entry_amount: entry_price * entry_quantity,
          timestamp,
        }
      })}).collect(),
    };
    Ok(Json(serde_json::json!(response)))
  }

  pub fn routes(&self) -> Router {
    let ctx = self.ctx.clone();
    return Router::new()
      .route("/", get(Self::gets))
      .with_state(ctx);
  }
}
