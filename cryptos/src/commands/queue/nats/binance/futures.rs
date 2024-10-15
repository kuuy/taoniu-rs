use std::collections::HashMap;

use async_nats::Subscriber;
use futures_util::{stream, StreamExt};

use clap::{Parser};

use crate::common::*;
use crate::queue::nats::workers::binance::futures::*;

#[derive(Parser)]
pub struct FuturesCommand {}

impl Default for FuturesCommand {
  fn default() -> Self {
    Self::new()
  }
}

impl FuturesCommand {
  pub fn new() -> Self {
    Self {
      ..Default::default()
    }
  }

  pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
    println!("queue nats binance futures");
    let rdb = Rdb::new(2).await?;
    let rmq = Rmq::new(2).await?;
    let pool = Pool::new(2)?;
    let nats = Nats::new().await?;
    let ctx = Ctx::new(rdb, rmq, pool, nats);

    let mut callbacks = HashMap::<&str, Vec<EventFn>>::new();
    FuturesWorker::new(ctx.clone()).subscribe(&mut callbacks).await?;

    let client = ctx.nats.clone();

    let mut subscribers: Vec<Subscriber> = Vec::new();
    for event in callbacks.keys() {
      subscribers.push(client.subscribe(&event[..]).await.unwrap());
    }

    let mut messages = stream::select_all(subscribers);
    let handle = tokio::spawn(Box::pin({
      let ctx = ctx.clone();
      async move {
        while let Some(message) = messages.next().await {
          if let Some(methods) = callbacks.get(&message.subject[..]) {
            for method in methods {
              let payload = std::str::from_utf8(&message.payload).unwrap();
              let _ = method(ctx.clone(), payload.into()).await;
            }
          }
        }
      }
    }));
    handle.await.expect("queue nats binance futures process failed.");

    Ok(())
  }
}
