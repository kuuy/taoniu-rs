pub const REDIS_KEY_BALANCE: &str = "binance:spot:balance";
pub const REDIS_KEY_CURRENCIES: &str = "binance:spot:currencies";
pub const REDIS_KEY_TICKERS: &str = "binance:spot:realtime";
pub const REDIS_KEY_TICKERS_FLUSH: &str = "binance:spot:tickers:flush";
pub const REDIS_KEY_KLINES: &str = "binance:spot:klines";
pub const REDIS_KEY_INDICATORS: &str = "binance:spot:indicators";
pub const REDIS_KEY_TRADINGS_LAST_PRICE: &str = "binance:spot:tradings:last:price";
// pub const REDIS_KEY_TRADINGS_TRIGGERS_PLACE: &str = "binance:spot:tradings:triggers:place";
pub const SCALPING_MIN_BINANCE: f64 = 50.0;
// pub const TRIGGERS_MIN_BINANCE: f64 = 50.0;
pub const RSMQ_QUEUE_TICKERS: &str  = "binance.spot.tickers";
pub const RSMQ_QUEUE_KLINES: &str = "binance.spot.klines";
pub const RSMQ_QUEUE_DEPTH: &str = "binance.spot.depth";
pub const RSMQ_QUEUE_INDICATORS: &str = "binance.spot.indicators";
pub const RSMQ_QUEUE_STRATEGIES: &str = "binance.spot.strategies";
pub const RSMQ_QUEUE_PLANS: &str = "binance.spot.plans";
pub const RSMQ_QUEUE_ACCOUNT: &str = "binance.spot.account";
pub const RSMQ_QUEUE_ORDERS: &str = "binance.spot.orders";
pub const RSMQ_QUEUE_POSITIONS: &str = "binance.spot.positions";
pub const RSMQ_QUEUE_TRADINGS_SCALPING: &str = "binance.spot.tradings.scalping";
pub const RSMQ_QUEUE_TRADINGS_TRIGGERS: &str = "binance.spot.tradings.triggers";
pub const RSMQ_JOBS_TICKERS_FLUSH: &str = "binance:spot:tickers:flush";
pub const RSMQ_JOBS_TICKERS_UPDATE: &str = "binance:spot:tickers:update";
pub const RSMQ_JOBS_KLINES_SYNC: &str = "binance:spot:klines:sync";
pub const RSMQ_JOBS_KLINES_CLEAN: &str = "binance:spot:klines:clean";
pub const RSMQ_JOBS_INDICATORS_FLUSH: &str = "binance:spot:indicators:flush";
pub const RSMQ_JOBS_STRATEGIES_FLUSH: &str = "binance:spot:strategies:flush";
pub const RSMQ_JOBS_PLANS_FLUSH: &str = "binance:spot:plans:flush";
pub const RSMQ_JOBS_ACCOUNT_FLUSH: &str = "binance:spot:account:flush";
pub const RSMQ_JOBS_ORDERS_OPEN: &str = "binance:spot:orders:open";
pub const RSMQ_JOBS_ORDERS_FLUSH: &str = "binance:spot:orders:flush";
pub const RSMQ_JOBS_ORDERS_SYNC: &str = "binance:spot:orders:sync";
pub const RSMQ_JOBS_TRADINGS_LAUNCHPAD_PLACE: &str = "binance:spot:tradings:launchpad:place";
pub const RSMQ_JOBS_TRADINGS_LAUNCHPAD_FLUSH: &str = "binance:spot:tradings:launchpad:flush";
pub const RSMQ_JOBS_TRADINGS_SCALPING_PLACE: &str = "binance:spot:tradings:scalping:place";
pub const RSMQ_JOBS_TRADINGS_SCALPING_FLUSH: &str = "binance:spot:tradings:scalping:flush";
pub const RSMQ_JOBS_TRADINGS_TRIGGERS_PLACE: &str = "binance:spot:tradings:triggers:place";
pub const RSMQ_JOBS_TRADINGS_TRIGGERS_FLUSH: &str = "binance:spot:tradings:triggers:flush";
pub const NATS_EVENTS_ACCOUNT_UPDATE: &str = "binance.spot.account.update";
pub const NATS_EVENTS_TICKERS_UPDATE: &str = "binance.spot.tickers.update";
pub const NATS_EVENTS_KLINES_UPDATE: &str = "binance.spot.klines.update";
pub const NATS_EVENTS_INDICATORS_UPDATE: &str = "binance.spot.indicators.update";
pub const NATS_EVENTS_STRATEGIES_UPDATE: &str = "binance.spot.strategies.update";
pub const NATS_EVENTS_PLANS_UPDATE: &str = "binance.spot.plans.update";
pub const NATS_EVENTS_ORDERS_UPDATE: &str = "binance.spot.orders.update";
pub const NATS_EVENTS_TRADINGS_SCALPING_PLACE: &str = "binance.spot.tradings.scalping.place";
pub const NATS_EVENTS_API_KLINES_FLUSH: &str = "binance.spot.api.klines.flush";
pub const STREAMS_API_KLINES_FLUSH: &str = "klines";
pub const LOCKS_ACCOUNT_FLUSH: &str = "locks:binance:spot:account:flush";
pub const LOCKS_SYMBOLS_FLUSH: &str = "locks:binance:spot:symbols:flush";
pub const LOCKS_KLINES_FLUSH: &str = "locks:binance:spot:klines:flush";
pub const LOCKS_KLINES_SYNC: &str = "locks:binance:spot:klines:sync";
pub const LOCKS_INDICATORS_FLUSH: &str = "locks:binance:spot:indicators:flush";
pub const LOCKS_STRATEGIES_FLUSH: &str = "locks:binance:spot:strategies:flush";
pub const LOCKS_PLANS_FLUSH: &str = "locks:binance:spot:plans:flush";
pub const LOCKS_TRADINGS_SCALPING_PLACE: &str = "locks:binance:spot:tradings:scalping:place";
pub const LOCKS_TRADINGS_SCALPING_FLUSH: &str = "locks:binance:spot:tradings:scalping:flush";
pub const LOCKS_TRADINGS_TRIGGERS_PLACE: &str = "locks:binance:spot:tradings:triggers:place";
pub const LOCKS_TRADINGS_TRIGGERS_FLUSH: &str = "locks:binance:spot:tradings:triggers:flush";
pub const LOCKS_TASKS_KLINES_FLUSH: &str = "locks:tasks:binance:spot:klines:flush";
pub const LOCKS_TASKS_KLINES_FIX: &str = "locks:tasks:binance:spot:klines:fix";
pub const LOCKS_STREAMS_API_KLINES_FLUSH: &str = "locks:streams:api:binance:spot:klines:flush";