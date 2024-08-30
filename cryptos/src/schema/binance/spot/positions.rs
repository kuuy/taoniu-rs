diesel::table! {
  #[sql_name = "binance_spot_positions"]
  scalping (id) {
    id -> Varchar,
    symbol -> Varchar,
    notional -> Double,
    entry_price -> Double,
    entry_quantity -> Double,
    entry_amount -> Double,
    timestamp -> BigInt,
    status -> Integer,
    version -> BigInt,
    created_at -> Timestamptz,
    updated_at -> Timestamptz,
  }
}