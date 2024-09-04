use diesel::prelude::*;

use crate::common::*;

#[derive(Default)]
pub struct IndicatorsRepository {}

impl IndicatorsRepository {
  pub async fn pivot<T>(
    ctx: Ctx,
    symbol: T,
    interval: T,
  ) -> Result<(), Box<dyn std::error::Error>> 
  where
    T: AsRef<str>
  {
    Ok(())
  }

  pub async fn atr<T>(
    ctx: Ctx,
    symbol: T,
    interval: T,
    period: i32,
    limit: i32,
  ) -> Result<(), Box<dyn std::error::Error>> 
  where
    T: AsRef<str>
  {
    Ok(())
  }

  pub async fn zlema<T>(
    ctx: Ctx,
    symbol: T,
    interval: T,
    period: i32,
    limit: i32,
  ) -> Result<(), Box<dyn std::error::Error>> 
  where
    T: AsRef<str>
  {
    Ok(())
  }

  pub async fn ha_zlema<T>(
    ctx: Ctx,
    symbol: T,
    interval: T,
    period: i32,
    limit: i32,
  ) -> Result<(), Box<dyn std::error::Error>> 
  where
    T: AsRef<str>
  {
    Ok(())
  }

  pub async fn kdj<T>(
    ctx: Ctx,
    symbol: T,
    interval: T,
    long_period: i32,
    short_period: i32,
    limit: i32,
  ) -> Result<(), Box<dyn std::error::Error>> 
  where
    T: AsRef<str>
  {
    Ok(())
  }

  pub async fn bbands<T>(
    ctx: Ctx,
    symbol: T,
    interval: T,
    period: i32,
    limit: i32,
  ) -> Result<(), Box<dyn std::error::Error>> 
  where
    T: AsRef<str>
  {
    Ok(())
  }

  pub async fn ichimoku_cloud<T>(
    ctx: Ctx,
    symbol: T,
    interval: T,
    tenkan_period: i32,
    kijun_period: i32,
    senkou_period: i32,
    limit: i32,
  ) -> Result<(), Box<dyn std::error::Error>> 
  where
    T: AsRef<str>
  {
    Ok(())
  }

  pub async fn volume_profile<T>(
    ctx: Ctx,
    symbol: T,
    interval: T,
    period: i32,
    limit: i32,
  ) -> Result<(), Box<dyn std::error::Error>> 
  where
    T: AsRef<str>
  {
    Ok(())
  }
}
