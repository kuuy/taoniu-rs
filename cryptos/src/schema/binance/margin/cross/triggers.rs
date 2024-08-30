diesel::table! {
  #[sql_name = "binance_margin_cross_triggers"]
  scalping (id) {
    id -> Varchar,
    symbol -> Varchar,
    side -> Integer,
    capital -> Double,
    price -> Double,
    take_price -> Double,
    stop_price -> Double,
    take_order_id -> BigInt,
    stop_order_id -> BigInt,
    profit -> Double,
    timestamp -> BigInt,
    status -> Integer,
    version -> BigInt,
    remark -> Varchar,
    expired_at -> Timestamptz,
    created_at -> Timestamptz,
    updated_at -> Timestamptz,
  }
}