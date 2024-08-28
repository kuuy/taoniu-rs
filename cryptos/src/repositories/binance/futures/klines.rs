use std::ops::Sub;
use std::time::Duration;

use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use chrono::{prelude::Utc, Timelike};

// use crate::common::*;
// use crate::models::binance::spot::kline::schema::dsl::*;

#[derive(Default)]
pub struct KlinesRepository {}

impl KlinesRepository {
  pub fn timestamp(&self, interval: String) -> i64 {
    let mut datetime = Utc::now();
    datetime = datetime.sub(Duration::from_secs(datetime.second() as u64));
    if interval == "15m" {
      let minutes = datetime.minute() as u64 - ((Decimal::from_u64(datetime.minute() as u64).unwrap() / dec!(15)).floor() * dec!(15)).to_u64().unwrap();
      datetime = datetime.sub(Duration::from_secs(minutes * 60));
    } else if interval == "4h" {
      let hours = datetime.hour() as u64 - ((Decimal::from_u64(datetime.hour() as u64).unwrap() / dec!(4)).floor() * dec!(4)).to_u64().unwrap();
      let minutes = datetime.minute() as u64;
      datetime = datetime.sub(Duration::from_secs(hours * 3600 + minutes * 60));
    } else if interval == "1d" {
      let hours = datetime.hour() as u64;
      let minutes = datetime.minute() as u64;
      datetime = datetime.sub(Duration::from_secs(hours * 3600 + minutes * 60));
    }
    datetime.timestamp_millis()
  }
}
