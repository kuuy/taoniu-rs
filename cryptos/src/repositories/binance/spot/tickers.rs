use crate::common::*;

#[derive(Default)]
pub struct TickersRepository {}

impl TickersRepository {
  pub async fn ranking<T>(
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
        local key = 'binance:spot:realtime:' .. KEYS[i]
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
        values.iter().enumerate().for_each(|(i, value)| {
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
      _ => {},
    }
    vars
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
    Ok(())
  }
}