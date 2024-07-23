use rocket::{serde::json::Json, response::status, State, get, post, delete};
use crate::protocol::protocol::{TableResponse, OrderResponse};
use crate::db::mysql::OrdersInput;
use crate::api::Storage;
use uuid::Uuid;

#[get("/tables")]
pub fn get_tables(storage: &State<Box<dyn Storage + Send + Sync>>) -> Result<Json<Vec<TableResponse>>, status::Custom<String>> {
    storage.get_tables()
        .map(Json)
        .map_err(|e| status::Custom(rocket::http::Status::InternalServerError, e))
}

#[get("/tables/<table_id>/orders")]
pub fn get_table_orders(table_id: u64, storage: &State<Box<dyn Storage + Send + Sync>>) -> Result<Json<Vec<OrderResponse>>, status::Custom<String>> {
    storage.get_table_orders(table_id)
        .map(Json)
        .map_err(|e| status::Custom(rocket::http::Status::InternalServerError, e))
}

#[get("/tables/<table_id>/orders/<order_id>")]
pub fn get_table_order(table_id: u64, order_id: String, storage: &State<Box<dyn Storage + Send + Sync>>) -> Result<Json<OrderResponse>, status::Custom<String>> {
    let uuid = Uuid::parse_str(&order_id).map_err(|e| status::Custom(rocket::http::Status::BadRequest, e.to_string()))?;
    storage.get_table_order(table_id, uuid)
        .map(Json)
        .map_err(|e| status::Custom(rocket::http::Status::InternalServerError, e))
}

#[post("/tables/<table_id>/orders", data = "<orders_data>")]
pub fn add_table_orders(table_id: u64, orders_data: Json<OrdersInput>, storage: &State<Box<dyn Storage + Send + Sync>>) -> Result<Json<Vec<Uuid>>, status::Custom<String>> {
    storage.add_table_orders(table_id, orders_data.into_inner())
        .map(Json)
        .map_err(|e| status::Custom(rocket::http::Status::InternalServerError, e))
}

#[delete("/tables/<table_id>/orders/<order_id>")]
pub fn delete_table_order(table_id: u64, order_id: String, storage: &State<Box<dyn Storage + Send + Sync>>) -> Result<status::NoContent, status::Custom<String>> {
    let uuid = Uuid::parse_str(&order_id).map_err(|e| status::Custom(rocket::http::Status::BadRequest, e.to_string()))?;
    storage.delete_table_order(table_id, uuid)
        .map(|_| status::NoContent)
        .map_err(|e| status::Custom(rocket::http::Status::InternalServerError, e))
}