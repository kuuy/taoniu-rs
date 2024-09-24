use serde::Serialize;

#[derive(Serialize)]
pub struct SuccessResponse {
  pub success: bool,
  pub data: Box<dyn erased_serde::Serialize>,
}

#[derive(Serialize)]
pub struct ErrorResponse<T> {
  pub success: bool,
  pub code: T,
  pub message: T,
}

impl<T> ErrorResponse<T>
where
  T: AsRef<str>
{
  pub fn new(success: bool, code: T, message: T) -> Self {
    Self {
      success: success,
      code: code,
      message: message,
    }
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