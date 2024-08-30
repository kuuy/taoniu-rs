diesel::table! {
  #[sql_name = "binance_futures_orders"]
  orders (id) {
    id -> Varchar,
    symbol -> Varchar,
    order_id -> BigInt,
    #[sql_name="type"]
    order_type -> Varchar,
    position_side -> Varchar,
    side -> Varchar,
    price -> Double,
    avg_price -> Double,
    activate_price -> Double,
    stop_price -> Double,
    price_rate -> Double,
    quantity -> Double,
    executed_quantity -> Double,
    open_time -> BigInt,
    update_time -> BigInt,
    working_type -> Varchar,
    price_protect -> Bool,
    reduce_only -> Bool,
    close_position -> Bool,
    status -> Varchar,
    remark -> Varchar,
    created_at -> Timestamptz,
    updated_at -> Timestamptz,
  }
}