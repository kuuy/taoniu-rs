diesel::table! {
  #[sql_name = "binance_spot_symbols"]
  symbols (id) {
    id -> Varchar,
    symbol -> Varchar,
    base_asset -> Varchar,
    quote_asset -> Varchar,
    filters -> Jsonb,
    depth -> Jsonb,
    is_spot -> Bool,
    is_margin -> Bool,
    status -> Varchar,
    created_at -> Timestamptz,
    updated_at -> Timestamptz,
  }
}