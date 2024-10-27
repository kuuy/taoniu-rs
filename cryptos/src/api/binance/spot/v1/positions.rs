use axum::{
  extract::State,
  routing::get,
  http::StatusCode,
  Json,
  Router,
};
use serde::Serialize;

use crate::common::*;
use crate::api::response::*;
use crate::repositories::binance::spot::positions::*;

pub struct PositionsRouter {
  ctx: Ctx,
}

#[derive(Serialize)]
pub struct PositionInfo {
  id: String,
  symbol: String,
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
  ) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let positions = match PositionsRepository::gets(ctx.clone()).await {
      Ok(result) => result,
      Err(_) => {
        let message = ErrorMessage::new(false, "500", "database error");
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!(message))))
      }
    };
    let response = RankingResponse{
      success: true,
      data: positions.into_iter().map(|x: (String, String, f64, f64, f64, f64, i64)| -> Box<dyn erased_serde::Serialize> { Box::new({
        let (id, symbol, notional, entry_price, entry_quantity,entry_amount, timestamp) = x;
        PositionInfo{
          id,
          symbol,
          notional,
          entry_price,
          entry_quantity,
          entry_amount,
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
