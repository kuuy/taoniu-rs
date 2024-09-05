pub const REDIS_KEY_BALANCE: &str = "binance:futures:balance";
pub const REDIS_KEY_CURRENCIES: &str = "binance:futures:currencies";
pub const REDIS_KEY_TICKERS: &str = "binance:futures:realtime";
pub const REDIS_KEY_KLINES: &str = "binance:futures:klines";
pub const REDIS_KEY_INDICATORS: &str = "binance:futures:indicators";
// pub const REDIS_KEY_KLINES: &str = "binance:futures:klines";
// pub const REDIS_KEY_TRADINGS_LAST_PRICE: &str = "binance:futures:tradings:last:price:%v";
// pub const REDIS_KEY_TRADINGS_TRIGGERS_PLACE: &str = "binance:futures:tradings:triggers:place:%v";
// pub const SCALPING_MIN_BINANCE: i32 = 50;
// pub const TRIGGERS_MIN_BINANCE: i32 = 50;
pub const RSMQ_QUEUE_TICKERS: &str  = "binance.futures.tickers";
pub const RSMQ_QUEUE_KLINES: &str = "binance.futures.klines";
pub const RSMQ_QUEUE_DEPTH: &str = "binance.futures.depth";
pub const RSMQ_QUEUE_INDICATORS: &str = "binance.futures.indicators";
pub const RSMQ_QUEUE_STRATEGIES: &str = "binance.futures.strategies";
pub const RSMQ_QUEUE_PLANS: &str = "binance.futures.plans";
pub const RSMQ_QUEUE_ACCOUNT: &str = "binance.futures.account";
pub const RSMQ_QUEUE_ORDERS: &str = "binance.futures.orders";
pub const RSMQ_QUEUE_POSITIONS: &str = "binance.futures.positions";
pub const RSMQ_QUEUE_TRADINGS_SCALPING: &str = "binance.futures.tradings.scalping";
pub const RSMQ_QUEUE_TRADINGS_TRIGGERS: &str = "binance.futures.tradings.triggers";
pub const RSMQ_JOBS_TICKERS_FLUSH: &str = "binance:futures:tickers:flush";
pub const RSMQ_JOBS_TICKERS_UPDATE: &str = "binance:futures:tickers:update";
pub const RSMQ_JOBS_KLINES_FLUSH: &str = "binance:futures:klines:flush";
pub const RSMQ_JOBS_KLINES_CLEAN: &str = "binance:futures:klines:clean";
pub const RSMQ_JOBS_ACCOUNT_FLUSH: &str = "binance:futures:account:flush";
pub const RSMQ_JOBS_ORDERS_OPEN: &str = "binance:futures:orders:open";
pub const RSMQ_JOBS_ORDERS_FLUSH: &str = "binance:futures:orders:flush";
pub const RSMQ_JOBS_ORDERS_SYNC: &str = "binance:futures:orders:sync";
pub const RSMQ_JOBS_TRADINGS_LAUNCHPAD_PLACE: &str = "binance:futures:tradings:launchpad:place";
pub const RSMQ_JOBS_TRADINGS_LAUNCHPAD_FLUSH: &str = "binance:futures:tradings:launchpad:flush";
pub const RSMQ_JOBS_TRADINGS_SCALPING_PLACE: &str = "binance:futures:tradings:scalping:place";
pub const RSMQ_JOBS_TRADINGS_SCALPING_FLUSH: &str = "binance:futures:tradings:scalping:flush";
pub const RSMQ_JOBS_TRADINGS_TRIGGERS_PLACE: &str = "binance:futures:tradings:triggers:place";
pub const RSMQ_JOBS_TRADINGS_TRIGGERS_FLUSH: &str = "binance:futures:tradings:triggers:flush";
pub const NATS_EVENTS_INDICATORS_UPDATE: &str = "binance.futures.indicators.update";
pub const NATS_EVENTS_STRATEGIES_UPDAT: &str = "binance.futures.strategies.update";
pub const NATS_EVENTS_PLANS_UPDATE: &str = "binance.futures.plans.update";
pub const NATS_EVENTS_ACCOUNT_UPDATE: &str = "binance.futures.account.update";
pub const NATS_EVENTS_ORDERS_UPDATE: &str = "binance.futures.orders.update";
pub const NATS_EVENTS_TICKERS_UPDATE: &str = "binance.futures.tickers.update";
pub const NATS_EVENTS_KLINES_UPDATE: &str = "binance.futures.klines.update";
pub const NATS_EVENTS_TRADINGS_SCALPING_PLACE: &str = "binance.futures.tradings.scalping.place";
pub const LOCKS_ACCOUNT_FLUSH: &str = "locks:binance:futures:account:flush";
pub const LOCKS_SYMBOLS_FLUSH: &str = "locks:binance:futures:symbols:flush";
pub const LOCKS_KLINES_FLUSH: &str = "locks:binance:futures:klines:flush";

// pub const REDIS_QUEUE_TICKERS: &str  = "binance.futures.tickers";