diesel::table! {
  #[sql_name = "binance_futures_analysis_tradings_scalping"]
  scalping (id) {
    id -> Varchar,
    side -> Integer,
    day -> Date,
    buys_count -> Double,
    sells_count -> Double,
    buys_amount -> Double,
    sells_amount -> Double,
    profit -> Double,
    additive_profit -> Double,
    created_at -> Timestamptz,
    updated_at -> Timestamptz,
  }
}