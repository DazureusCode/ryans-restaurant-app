pub mod tables;

use crate::protocol::protocol::{TableResponse, OrderResponse};
use uuid::Uuid;
use crate::db::mysql::OrdersInput;

pub trait Storage: Send + Sync {
    fn get_tables(&self) -> Result<Vec<TableResponse>, String>;
    fn get_table_orders(&self, table_id: u64) -> Result<Vec<OrderResponse>, String>;
    fn get_table_order(&self, table_id: u64, order_id: Uuid) -> Result<OrderResponse, String>;
    fn add_table_orders(&self, table_id: u64, orders: OrdersInput) -> Result<Vec<Uuid>, String>;
    fn delete_table_order(&self, table_id: u64, order_id: Uuid) -> Result<(), String>;
}