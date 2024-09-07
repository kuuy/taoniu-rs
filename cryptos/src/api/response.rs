use std::any::Any;
use std::fmt::Debug;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

#[typetag::serde(tag = "data")]
pub trait DataItem: Debug {}

#[derive(Serialize, Deserialize, Debug)]
pub struct SuccessResponse {
  pub success: bool,
  pub data: Box<dyn DataItem>,
}

impl SuccessResponse {
  pub fn new(success: bool, data: Box<dyn DataItem>) -> Self {
    Self {
      success: success,
      data: data,
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct PagenateResponse {
  pub success: bool,
  pub data: Vec<Box<dyn DataItem>>,
  pub total: u64,
  pub current: u32,
  pub page_size: u32,
}

impl PagenateResponse {
  pub fn new(success: bool, data: Vec<Box<dyn DataItem>>, total: u64, current: u32, page_size: u32) -> Self {
    Self {
      success: success,
      data: data,
      total: total,
      current: current,
      page_size: page_size,
    }
  }
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