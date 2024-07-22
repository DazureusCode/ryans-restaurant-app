use rocket::{serde::json::Json, response::status, State, get, post, delete};
use crate::protocol::protocol::{TableResponse, OrderResponse};
use crate::domain::tables::{get_all_tables, get_orders, get_order, add_orders, remove_order};
use crate::db::mysql::{MySqlDb, OrdersInput};
use uuid::Uuid;

#[get("/tables")]
pub fn get_tables(db: &State<MySqlDb>) -> Result<Json<Vec<TableResponse>>, status::Custom<String>> {
    get_all_tables(db)
        .map(Json)
        .map_err(|e| status::Custom(rocket::http::Status::InternalServerError, e))
}

#[get("/tables/<table_id>/orders")]
pub fn get_table_orders(table_id: u64, db: &State<MySqlDb>) -> Result<Json<Vec<OrderResponse>>, status::Custom<String>> {
    get_orders(table_id, db)
        .map(Json)
        .map_err(|e| status::Custom(rocket::http::Status::InternalServerError, e))
}

#[get("/tables/<table_id>/orders/<order_id>")]
pub fn get_table_order(table_id: u64, order_id: String, db: &State<MySqlDb>) -> Result<Json<OrderResponse>, status::Custom<String>> {
    let uuid = Uuid::parse_str(&order_id).map_err(|e| status::Custom(rocket::http::Status::BadRequest, e.to_string()))?;
    get_order(table_id, uuid, db)
        .map(Json)
        .map_err(|e| status::Custom(rocket::http::Status::InternalServerError, e))
}

#[post("/tables/<table_id>/orders", data = "<orders_data>")]
pub fn add_table_orders(table_id: u64, orders_data: Json<OrdersInput>, db: &State<MySqlDb>) -> Result<Json<Vec<Uuid>>, status::Custom<String>> {
    add_orders(table_id, orders_data.into_inner(), db)
        .map(Json)
        .map_err(|e| status::Custom(rocket::http::Status::InternalServerError, e))
}

#[delete("/tables/<table_id>/orders/<order_id>")]
pub fn delete_table_order(table_id: u64, order_id: String, db: &State<MySqlDb>) -> Result<status::NoContent, status::Custom<String>> {
    let uuid = Uuid::parse_str(&order_id).map_err(|e| status::Custom(rocket::http::Status::BadRequest, e.to_string()))?;
    remove_order(table_id, uuid, db)
        .map(|_| status::NoContent)
        .map_err(|e| status::Custom(rocket::http::Status::InternalServerError, e))
}
