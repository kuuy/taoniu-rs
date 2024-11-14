use chrono::NaiveDate;

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
use crate::repositories::binance::futures::analysis::tradings::triggers::*;

#[derive(Deserialize)]
struct ListingsRequest {
  current: u32,
  page_size: u32,
}

#[derive(Serialize)]
pub struct TriggerInfo {
  id: String,
  day: NaiveDate,
  buys_count: i32,
  sells_count: i32,
  buys_amount: f64,
  sells_amount: f64,
  profit: f64,
  additive_profit: f64,
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

    let total = match TriggersRepository::count(ctx.clone()).await {
      Ok(result) => result,
      Err(_) => {
        let message = ErrorMessage::new(false, "500", "database error");
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!(message))))
      },
    };
    let analysis = match TriggersRepository::listings(
      ctx.clone(),
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
      data: analysis.into_iter().map(|x: (String, NaiveDate, i32, i32, f64, f64, f64, f64)| -> Box<dyn erased_serde::Serialize> { Box::new({
        let (id, day, buys_count, sells_count, buys_amount, sells_amount, profit, additive_profit) = x;
        TriggerInfo{
          id,
          day,
          buys_count,
          sells_count,
          buys_amount,
          sells_amount,
          profit,
          additive_profit,
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
