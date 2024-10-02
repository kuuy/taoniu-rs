use std::time::Duration;
use futures_util::StreamExt;

use crate::common::*;
use crate::config::binance::futures::config as Config;
use crate::queue::nats::payload::binance::futures::klines::*;
use crate::queue::nats::jobs::binance::futures::indicators::*;
use crate::repositories::binance::futures::indicators::*;

pub struct IndicatorsWorker {
  ctx: Ctx,
}

impl IndicatorsWorker {
  pub fn new(ctx: Ctx) -> Self {
    Self {
      ctx: ctx,
    }
  }

  pub async fn pivot<T>(ctx: Ctx, symbol: T, interval: T) -> Result<(), Box<dyn std::error::Error>>
  where
    T: AsRef<str>
  {
    let symbol = symbol.as_ref();
    let interval = interval.as_ref();

    println!("binance futures indicators nats workers pivot {symbol:} {interval:}");
    let _ = IndicatorsRepository::pivot(
      ctx.clone(),
      symbol,
      interval,
    ).await?;

    Ok(())
  }

  pub async fn atr<T>(ctx: Ctx, symbol: T, interval: T) -> Result<(), Box<dyn std::error::Error>>
  where
    T: AsRef<str>
  {
    let symbol = symbol.as_ref();
    let interval = interval.as_ref();

    println!("binance futures indicators nats workers atr {symbol:} {interval:}");
    let _ = IndicatorsRepository::atr(
      ctx.clone(),
      symbol,
      interval,
      14,
      100,
    ).await;

    Ok(())
  }

  pub async fn zlema<T>(ctx: Ctx, symbol: T, interval: T) -> Result<(), Box<dyn std::error::Error>>
  where
    T: AsRef<str>
  {
    let symbol = symbol.as_ref();
    let interval = interval.as_ref();

    println!("binance futures indicators nats workers zlema {symbol:} {interval:}");
    let _ = IndicatorsRepository::zlema(
      ctx.clone(),
      symbol,
      interval,
      14,
      100,
    ).await;

    Ok(())
  }

  pub async fn ha_zlema<T>(ctx: Ctx, symbol: T, interval: T) -> Result<(), Box<dyn std::error::Error>>
  where
    T: AsRef<str>
  {
    let symbol = symbol.as_ref();
    let interval = interval.as_ref();

    println!("binance futures indicators nats workers ha zlema {symbol:} {interval:}");
    let _ = IndicatorsRepository::ha_zlema(
      ctx.clone(),
      symbol,
      interval,
      14,
      100,
    ).await;

    Ok(())
  }

  pub async fn kdj<T>(ctx: Ctx, symbol: T, interval: T) -> Result<(), Box<dyn std::error::Error>>
  where
    T: AsRef<str>
  {
    let symbol = symbol.as_ref();
    let interval = interval.as_ref();

    println!("binance futures indicators nats workers kdj {symbol:} {interval:}");
    let _ = IndicatorsRepository::kdj(
      ctx.clone(),
      symbol,
      interval,
      9,
      3,
      100,
    ).await;

    Ok(())
  }

  pub async fn bbands<T>(ctx: Ctx, symbol: T, interval: T) -> Result<(), Box<dyn std::error::Error>>
  where
    T: AsRef<str>
  {
    let symbol = symbol.as_ref();
    let interval = interval.as_ref();

    println!("binance futures indicators nats workers bbands {symbol:} {interval:}");
    let _ = IndicatorsRepository::bbands(
      ctx.clone(),
      symbol,
      interval,
      14,
      100,
    ).await;

    Ok(())
  }

  pub async fn ichimoku_cloud<T>(ctx: Ctx, symbol: T, interval: T) -> Result<(), Box<dyn std::error::Error>>
  where
    T: AsRef<str>
  {
    let symbol = symbol.as_ref();
    let interval = interval.as_ref();

    let tenkan_period: i32;
    let kijun_period: i32;
    let senkou_period: i32;
    let limit: i64;
    if interval == "1m" {
      tenkan_period = 129;
      kijun_period = 374;
      senkou_period = 748;
      limit = 1440;
    } else if interval == "15m" {
      tenkan_period = 60;
      kijun_period = 174;
      senkou_period = 349;
      limit = 672;
    } else if interval == "4h" {
      tenkan_period = 11;
      kijun_period = 32;
      senkou_period = 65;
      limit = 126;
    } else {
      tenkan_period = 9;
      kijun_period = 26;
      senkou_period = 52;
      limit = 100;
    }
    println!("binance futures indicators nats workers ichimoku cloud {symbol:} {interval:}");
    let _ = IndicatorsRepository::ichimoku_cloud(
      ctx.clone(),
      symbol,
      interval,
      tenkan_period,
      kijun_period,
      senkou_period,
      limit,
    ).await;

    Ok(())
  }

  pub async fn volume_profile<T>(ctx: Ctx, symbol: T, interval: T) -> Result<(), Box<dyn std::error::Error>>
  where
    T: AsRef<str>
  {
    let symbol = symbol.as_ref();
    let interval = interval.as_ref();

    let limit: i64;
    if interval == "1m" {
      limit = 1440
    } else if interval == "15m" {
      limit = 672
    } else if interval == "4h" {
      limit = 126
    } else {
      limit = 100
    }

    println!("binance futures indicators nats workers volume profile {symbol:} {interval:}");
    let _ = IndicatorsRepository::volume_profile(
      ctx.clone(),
      symbol,
      interval,
      limit,
    ).await;

    Ok(())
  }

  pub async fn andean_oscillator<T>(ctx: Ctx, symbol: T, interval: T) -> Result<(), Box<dyn std::error::Error>>
  where
    T: AsRef<str>
  {
    let symbol = symbol.as_ref();
    let interval = interval.as_ref();

    let limit: i64;
    if interval == "1m" {
      limit = 1440
    } else if interval == "15m" {
      limit = 672
    } else if interval == "4h" {
      limit = 126
    } else {
      limit = 100
    }

    println!("binance futures indicators nats workers andean oscillator {symbol:} {interval:}");
    let _ = IndicatorsRepository::andean_oscillator(ctx.clone(), symbol, interval, 50, 9, limit).await;

    Ok(())
  }

  pub async fn process<T>(ctx: Ctx, symbol: T, interval: T) -> Result<(), Box<dyn std::error::Error>> 
  where
    T: AsRef<str>
  {
    let symbol = symbol.as_ref();
    let interval = interval.as_ref();

    let rdb = ctx.rdb.lock().await.clone();
    let mutex_id = xid::new().to_string();
    let redis_lock_key = format!("{}:{}:{}", Config::LOCKS_INDICATORS_FLUSH, interval, symbol);
    let mut mutex = Mutex::new(
      rdb,
      &redis_lock_key,
      &mutex_id,
    );
    if !mutex.lock(Duration::from_secs(30)).await.unwrap() {
      return Err(Box::from(format!("mutex failed {}", redis_lock_key)));
    }

    println!("binance futures indicators nats workers process {symbol:} {interval:}");
    Self::pivot(ctx.clone(), symbol, interval).await?;
    Self::atr(ctx.clone(), symbol, interval).await?;
    Self::zlema(ctx.clone(), symbol, interval).await?;
    Self::ha_zlema(ctx.clone(), symbol, interval).await?;
    Self::kdj(ctx.clone(), symbol, interval).await?;
    Self::bbands(ctx.clone(), symbol, interval).await?;
    Self::ichimoku_cloud(ctx.clone(), symbol, interval).await?;
    Self::volume_profile(ctx.clone(), symbol, interval).await?;
    Self::andean_oscillator(ctx.clone(), symbol, interval).await?;

    let job = IndicatorsJob::new(ctx.clone());
    let _ = job.update(symbol, interval).await;

    mutex.unlock().await.unwrap();

    Ok(())
  }

  pub async fn subscribe(&self) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance futures indicators nats workers subscribe");

    tokio::spawn(Box::pin({
      let ctx = self.ctx.clone();
      async move {
        let client = ctx.nats.clone();
        let mut subscriber = client.subscribe(Config::NATS_EVENTS_KLINES_UPDATE).await.unwrap();
        while let Some(message) = subscriber.next().await {
          if let Ok(payload) = serde_json::from_slice::<KlinesUpdatePayload<&str>>(message.payload.as_ref()) {
            if let Err(e) = Self::process(ctx.clone(), payload.symbol, payload.interval).await {
              println!("nats worders binance futures indicators process failed {} {} {:?}", payload.symbol, payload.interval, e);
            }
          }
        }
      }
    }));

    Ok(())
  }
}