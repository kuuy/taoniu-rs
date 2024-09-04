diesel::table! {
  #[sql_name = "binance_spot_orders"]
  orders (id) {
    id -> Varchar,
    symbol -> Varchar,
    order_id -> BigInt,
    #[sql_name="type"]
    order_type -> Varchar,
    side -> Varchar,
    price -> Double,
    avg_price -> Double,
    stop_price -> Double,
    quantity -> Double,
    executed_quantity -> Double,
    open_time -> BigInt,
    update_time -> BigInt,
    status -> Varchar,
    remark -> Varchar,
    created_at -> Timestamptz,
    updated_at -> Timestamptz,
  }
}