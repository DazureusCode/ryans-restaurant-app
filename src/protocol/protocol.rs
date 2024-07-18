use std::collections::HashMap;
use uuid::Uuid;
use serde::Serialize;

#[derive(Serialize)]
pub struct TableResponse {
    pub id: u64,
    pub orders: HashMap<Uuid, OrderResponse>,
}

#[derive(Serialize)]
pub struct OrderResponse {
    pub id: Uuid,
    pub menu_item: String,
    pub cooking_time: String,
}