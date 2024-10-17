use axum::{
  http::StatusCode,
  Json,
  response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Serialize)]
pub struct SuccessResponse {
  pub success: bool,
  pub data: Box<dyn erased_serde::Serialize>,
}

#[derive(Serialize)]
pub struct ErrorMessage<T> {
  pub success: bool,
  pub code: T,
  pub message: T,
}

impl<T> ErrorMessage<T>
where
  T: AsRef<str>
{
  pub fn new(success: bool, code: T, message: T) -> Self {
    Self {
      success,
      code,
      message,
    }
  }
}

#[derive(Serialize)]
pub struct ErrorResponse<T> {
  pub success: bool,
  pub message: ErrorMessage<T>,
}

impl<T> ErrorResponse<T>
where
  T: AsRef<str>
{
  pub fn json(status: StatusCode, code: T, message: T) -> Response {
    let code = code.as_ref();
    let message = message.as_ref();
    let message = ErrorMessage::new(false, code, message);
    (status, Json(serde_json::json!(message))).into_response()
  }
}

#[derive(Serialize)]
pub struct PagenateResponse {
  pub success: bool,
  pub data: Vec<Box<dyn erased_serde::Serialize>>,
  pub total: u64,
  pub current: u32,
  pub page_size: u32,
}

#[derive(Serialize)]
pub struct RankingResponse {
  pub success: bool,
  pub data: Vec<Box<dyn erased_serde::Serialize>>,
}

#[derive(Serialize)]
pub struct JweResponse<T> {
  pub payload: T,
}

impl<T> JweResponse<T>
where
  T: AsRef<str>
{
  pub fn new(payload: T) -> Self {
    Self {
      payload: payload,
    }
  }
}