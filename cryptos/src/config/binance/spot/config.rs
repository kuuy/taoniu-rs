pub const REDIS_KEY_TICKERS: &str = "binance:spot:realtime";
pub const REDIS_KEY_KLINES: &str = "binance:spot:klines";
// pub const REDIS_KEY_TRADINGS_LAST_PRICE: &str = "binance:spot:tradings:last:price";
// pub const REDIS_KEY_TRADINGS_TRIGGERS_PLACE: &str = "binance:spot:tradings:triggers:place";
// pub const SCALPING_MIN_BINANCE: i32 = 50;
// pub const TRIGGERS_MIN_BINANCE: i32 = 50;
pub const NATS_INDICATORS_UPDATE: &str = "binance.spot.indicators.update";
pub const NATS_STRATEGIES_UPDAT: &str = "binance.spot.strategies.update";
pub const NATS_PLANS_UPDATE: &str = "binance.spot.plans.update";
pub const NATS_ACCOUNT_UPDATE: &str = "binance.spot.account.update";
pub const NATS_ORDERS_UPDATE: &str = "binance.spot.orders.update";
pub const NATS_TICKERS_UPDATE: &str = "binance.spot.tickers.update";
pub const NATS_KLINES_UPDATE: &str = "binance.spot.klines.update";
pub const NATS_TRADINGS_SCALPING_PLACE: &str = "binance.spot.tradings.scalping.place";
pub const LOCKS_TASKS_SYMBOLS_FLUSH: &str = "locks:binance:spot:tasks:symbols:flush";

// pub const REDIS_QUEUE_TICKERS: &str  = "binance.spot.tickers";