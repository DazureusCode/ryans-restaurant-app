use serde::Serialize;
use uuid::Uuid;
use std::collections::HashMap;
use rocket::serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct OrderInput {
    pub menu_item: String,
}

#[derive(Debug, Deserialize)]
pub struct OrdersInput {
    pub orders: Vec<OrderInput>,
}

#[derive(Debug, Serialize)]
pub struct TableResponse {
    pub id: u64,
    pub orders: HashMap<Uuid, OrderResponse>,
}

#[derive(Debug, Serialize)]
pub struct OrderResponse {
    pub id: Uuid,
    pub menu_item: String,
    pub cooking_time: String,
}