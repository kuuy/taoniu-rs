diesel::table! {
  #[sql_name = "binance_spot_analysis_tradings_triggers"]
  triggers (id) {
    id -> Varchar,
    day -> Date,
    buys_count -> Integer,
    sells_count -> Integer,
    buys_amount -> Double,
    sells_amount -> Double,
    profit -> Double,
    additive_profit -> Double,
    created_at -> Timestamptz,
    updated_at -> Timestamptz,
  }
}