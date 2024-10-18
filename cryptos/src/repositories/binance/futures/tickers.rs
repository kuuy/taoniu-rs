use chrono::prelude::Utc;
use redis::AsyncCommands;

use crate::common::*;
use crate::config::binance::futures::config as Config;

#[derive(Default)]
pub struct TickersRepository {}

impl TickersRepository {
  pub async fn gets<T>(
    ctx: Ctx,
    symbols: Vec<T>,
    fields: Vec<T>,
  ) -> Vec<String>
  where
    T: AsRef<str>
  {
    let symbols = symbols.iter().map(|s| s.as_ref()).collect::<Vec<&str>>();
    let fields = fields.iter().map(|s| s.as_ref()).collect::<Vec<&str>>();
    let script = redis::Script::new(r"
      local hmget = function (key)
        local hash = {}
        local data = redis.call('HMGET', key, unpack(ARGV))
        for i = 1, #ARGV do
          hash[i] = data[i]
        end
        return hash
      end
      local data = {}
      for i = 1, #KEYS do
        local key = 'binance:futures:realtime:' .. KEYS[i]
        if redis.call('EXISTS', key) == 0 then
          data[i] = false
        else
          data[i] = hmget(key)
        end
      end
      return data
    ");
    let mut rdb = ctx.rdb.lock().await.clone();
    let mut vars = Vec::new();
    match script
      .key(symbols.as_slice())
      .arg(fields.as_slice())
      .invoke_async::<_, Vec<redis::Value>>(&mut rdb).await {
      Ok(values) => {
        values.iter().enumerate().for_each(|(_, value)| {
          if let redis::Value::Bulk(bulk) = value {
            let mut var = Vec::new();
            bulk.iter().for_each(|item| {
              if let redis::Value::Data(v) = item {
                let v = std::str::from_utf8(v).unwrap();
                var.push(v);
              }
            });
            vars.push(var.join(","));
          }
        })
      }
      _ => (),
    }
    vars
  }

  pub async fn ranking<T>(
    ctx: Ctx,
    symbols: Vec<T>,
    fields: Vec<T>,
    sort_field: T,
    sort_type: i32,
    current: u32,
    page_size: u32,
  ) -> Vec<String>
  where
    T: AsRef<str>
  {
    let symbols = symbols.iter().map(|s| s.as_ref()).collect::<Vec<&str>>();
    let fields = fields.iter().map(|s| s.as_ref()).collect::<Vec<&str>>();
    let sort_field = sort_field.as_ref();
    let sort_position: usize = fields.iter().position(|&v| v == sort_field).unwrap();

    let values = Self::gets(ctx.clone(), symbols.clone(), fields).await;
    let mut scores = Vec::new();
    for (i, v) in values.iter().enumerate() {
      let v: Vec<&str> = v.split(",").collect();
      let v = v[sort_position].parse::<f64>().unwrap();
      scores.push((i, v));
    }
    scores.sort_by(|a, b| {
      if sort_type == -1 {
        b.1.partial_cmp(&a.1).unwrap()
      } else {
        a.1.partial_cmp(&b.1).unwrap()
      }
    });

    let total = symbols.len();
    let current = current as usize;
    let page_size = page_size as usize;
    let offset = (current - 1) * page_size;

    if offset >= total {
      return vec![]
    }

    let (_, tail) = scores.split_at(offset);
    scores = Vec::from(tail);

    if page_size < scores.len() {
      let (head, _) = scores.split_at(offset + page_size);
      scores = Vec::from(head);
    }

    let mut ranking: Vec<String> = Vec::new();
    for v in scores.iter() {
      let symbol = symbols[v.0].to_string();
      ranking.push(format!("{},{}", &symbol[..], &values[v.0][..]));
    }

    ranking
  }

  pub async fn price<T>(
    ctx: Ctx,
    symbol: T,
  ) -> Result<f64, Box<dyn std::error::Error>>
  where
    T: AsRef<str>
  {
    let symbol = symbol.as_ref();

    let timestamp = Utc::now().timestamp_millis();
    let redis_key = format!("{}:{}", Config::REDIS_KEY_TICKERS, symbol);

    let mut rdb = ctx.rdb.lock().await.clone();
    let fields = vec!["price", "timestamp"];

    let (price, lasttime): (Option<f64>, Option<i64>) = match redis::cmd("HMGET")
      .arg(&redis_key)
      .arg(&fields)
      .query_async(&mut rdb)
      .await
    {
      Ok((Some(price), Some(lasttime))) => (price, lasttime),
      Ok(_) => return Err(Box::from(format!("ticker of {symbol:} not exists"))),
      Err(err) => return Err(err.into()),
    };

    let price = price.unwrap();
    let lasttime = lasttime.unwrap();

    if timestamp-lasttime > 30000 {
      rdb.zadd(Config::REDIS_KEY_TICKERS_FLUSH, symbol, timestamp).await?;
      return Err(Box::from(format!("ticker of {symbol:} has been expired")))
    }

    Ok(price)
  }

  pub async fn flush<T>(
    ctx: Ctx,
    symbols: Vec<T>,
  ) -> Result<(), Box<dyn std::error::Error>>
  where
    T: AsRef<str>
  {
    let symbols = symbols.iter().map(|s| s.as_ref()).collect::<Vec<&str>>();
    println!("tickers flush {symbols:?}");
    let _ = ctx.clone();
    Ok(())
  }
}