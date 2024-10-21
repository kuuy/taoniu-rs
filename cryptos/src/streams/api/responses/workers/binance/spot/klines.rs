use std::collections::HashMap;

use crate::common::*;
use crate::config::binance::spot::config as Config;

pub struct KlinesWorker {}

impl KlinesWorker {
  pub fn new(_: Ctx) -> Self {
    Self {}
  }

  pub async fn flush<T>(
    ctx: Ctx,
    payload: T,
  ) -> Result<(), Box<dyn std::error::Error>> 
  where
    T: AsRef<str>
  {
    let _ = ctx.clone();
    let _ = payload.as_ref();
    println!("binance spot streams api responses klines flush workers process");
    // let result = match serde_json::from_str::<Vec<(i64, &str, &str, &str, &str, &str, u32, &str, &str, &str, &str, &str)>>(payload) {
    //   Ok(Vec<(timestamp, open, high, low, close, volume,_, quota, _, _, _, _)>) => KlinesFlushPayload{
    //     open: open.parse::<f64>().unwrap(),
    //     close: close.parse::<f64>().unwrap(),
    //     high: high.parse::<f64>().unwrap(),
    //     low: low.parse::<f64>().unwrap(),
    //     volume: volume.parse::<f64>().unwrap(),
    //     quota: quota.parse::<f64>().unwrap(),
    //     timestamp: timestamp,
    //   },
    //   Err(err) => return Err(err.into()),
    // };

    Ok(())
  }

  pub async fn subscribe(
    &self,
    callbacks: &mut HashMap<&str, EventFn>,
  ) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance spot streams api responses workers subscribe");
    callbacks.insert(
      Config::STREAMS_API_KLINES_FLUSH,
      Box::new(|ctx, payload| Box::pin(Self::flush(ctx.clone(), payload))),
    );
    Ok(())
  }
}