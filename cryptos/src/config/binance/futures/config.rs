pub const REDIS_KEY_TICKERS: &str = "binance:futures:realtime";
pub const REDIS_KEY_TRADINGS_LAST_PRICE: &str = "binance:futures:tradings:last:price:%v";
pub const REDIS_KEY_TRADINGS_TRIGGERS_PLACE: &str = "binance:futures:tradings:triggers:place:%v";
pub const SCALPING_MIN_BINANCE: i32 = 50;
pub const TRIGGERS_MIN_BINANCE: i32 = 50;
pub const LOCKS_TASKS_SYMBOLS_FLUSH: &str = "locks:binance:futures:tasks:symbols:flush";

pub const REDIS_QUEUE_TICKERS: &str  = "binance.futures.tickers";