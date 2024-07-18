use rocket::{serde::json::Json, response::status, State, get, post, delete};
use crate::protocol::protocol::{TableResponse, OrderResponse};
use crate::domain::tables::{get_all_tables, get_orders, get_order, add_order, remove_order};
use crate::db::memdb::MemDb;
use uuid::Uuid;

#[get("/tables")]
pub fn get_tables(db: &State<MemDb>) -> Result<Json<Vec<TableResponse>>, status::Custom<String>> {
    get_all_tables(db)
        .map(Json)
        .map_err(|e| status::Custom(rocket::http::Status::InternalServerError, e))
}

#[get("/tables/<table_id>/orders")]
pub fn get_table_orders(table_id: u64, db: &State<MemDb>) -> Result<Json<Vec<OrderResponse>>, status::Custom<String>> {
    get_orders(table_id, db)
        .map(Json)
        .map_err(|e| status::Custom(rocket::http::Status::InternalServerError, e))
}

#[get("/tables/<table_id>/orders/<order_id>")]
pub fn get_table_order(table_id: u64, order_id: String, db: &State<MemDb>) -> Result<Json<OrderResponse>, status::Custom<String>> {
    let uuid = Uuid::parse_str(&order_id).map_err(|e| status::Custom(rocket::http::Status::BadRequest, e.to_string()))?;
    get_order(table_id, uuid, db)
        .map(Json)
        .map_err(|e| status::Custom(rocket::http::Status::InternalServerError, e))
}

#[post("/tables/<table_id>/orders", data = "<menu_item>")]
pub fn add_table_order(table_id: u64, menu_item: String, db: &State<MemDb>) -> Result<Json<Uuid>, status::Custom<String>> {
    add_order(table_id, menu_item, db)
        .map(Json)
        .map_err(|e| status::Custom(rocket::http::Status::InternalServerError, e))
}

#[delete("/tables/<table_id>/orders/<order_id>")]
pub fn delete_table_order(table_id: u64, order_id: String, db: &State<MemDb>) -> Result<status::NoContent, status::Custom<String>> {
    let uuid = Uuid::parse_str(&order_id).map_err(|e| status::Custom(rocket::http::Status::BadRequest, e.to_string()))?;
    remove_order(table_id, uuid, db)
        .map(|_| status::NoContent)
        .map_err(|e| status::Custom(rocket::http::Status::InternalServerError, e))
}