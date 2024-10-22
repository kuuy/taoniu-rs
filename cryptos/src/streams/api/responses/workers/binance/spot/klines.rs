use std::collections::HashMap;

use diesel::prelude::*;

use crate::common::*;
use crate::config::binance::spot::config as Config;
use crate::models::binance::spot::kline::*;
use crate::schema::binance::spot::klines::*;
use crate::repositories::binance::spot::klines::*;
use crate::streams::api::responses::payload::binance::spot::klines::*;

pub struct KlinesWorker {}

impl KlinesWorker {
  pub fn new(_: Ctx) -> Self {
    Self {}
  }

  pub async fn flush<T>(
    ctx: Ctx,
    params: Vec<T>,
    payload: T,
  ) -> Result<(), Box<dyn std::error::Error>> 
  where
    T: AsRef<str>
  {
    println!("binance spot streams api responses klines flush workers process");
    let params = params.iter().map(|s| s.as_ref()).collect::<Vec<&str>>();
    let payload = payload.as_ref();

    let symbol = params[0];
    let interval = params[1];

    let response = match serde_json::from_str::<KlinesFlushPayload<&str>>(payload.as_ref()) {
      Ok(result) => result,
      Err(err) => {
        println!("error: {}", err);
        return Err(err.into())
      },
    };

    for (timestamp, open, high, low, close, volume, _, quota, ..) in response.result.iter() {
      let open = open.parse::<f64>().unwrap();
      let close = close.parse::<f64>().unwrap();
      let high = high.parse::<f64>().unwrap();
      let low = low.parse::<f64>().unwrap();
      let volume = volume.parse::<f64>().unwrap();
      let quota = quota.parse::<f64>().unwrap();
      let timestamp = *timestamp;

      let kline: Option<Kline> = match KlinesRepository::get(ctx.clone(), symbol, interval, timestamp).await {
        Ok(Some(result)) => Some(result),
        Ok(None) => None,
        Err(err) => {
          println!("error {:?}", err);
          continue
        },
      };

      if kline.is_none() {
        let id = xid::new().to_string();
        KlinesRepository::create(
          ctx.clone(),
          id,
          symbol.to_string(),
          interval.to_string(),
          open,
          close,
          high,
          low,
          volume,
          quota,
          timestamp,
        ).await?;
      } else {
        KlinesRepository::update(
          ctx.clone(),
          kline.unwrap().id,
          (
            klines::open.eq(open),
            klines::close.eq(close),
            klines::high.eq(high),
            klines::low.eq(low),
            klines::volume.eq(volume),
            klines::quota.eq(quota),
          ),
        ).await?;
      }
    }

    Ok(())
  }

  pub async fn subscribe(
    &self,
    callbacks: &mut HashMap<&str, ResponseFn>,
  ) -> Result<(), Box<dyn std::error::Error>> {
    callbacks.insert(
      Config::STREAMS_API_KLINES_FLUSH,
      Box::new(|ctx, params, payload| Box::pin(Self::flush(ctx.clone(), params, payload))),
    );
    Ok(())
  }
}