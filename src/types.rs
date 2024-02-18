

use serde::Deserialize;

pub struct CSFloatParams {
    pub limit: i8,
    pub category: i8,
    pub max_price: i128,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CSFloatReturn {
    pub item: Item,
    pub id: String,
    pub price: i64,
    pub reference: Ref,
}

#[derive(Debug, Deserialize)]
pub struct Item {
  pub market_hash_name: String,
  pub wear_name: String,
  pub stickers: Option<Vec<Sticker>>,
}

#[derive(Debug, Deserialize)]
pub struct Sticker {
    pub sticker_id: i64
}

#[derive(Debug, Deserialize)]
pub struct Ref {
    pub base_price: i64,
    pub float_factor: f64,
    pub predicted_price: i64,
}