diesel::table! {
  #[sql_name = "binance_futures_tradings_triggers"]
  triggers (id) {
    id -> Varchar,
    symbol -> Varchar,
    trigger_id -> Varchar,
    buy_price -> Double,
    sell_price -> Double,
    buy_quantity -> Double,
    sell_quantity -> Double,
    buy_order_id -> BigInt,
    sell_order_id -> BigInt,
    status -> Integer,
    version -> BigInt,
    remark -> Varchar,
    created_at -> Timestamptz,
    updated_at -> Timestamptz,
  }
}