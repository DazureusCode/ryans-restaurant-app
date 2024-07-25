pub mod mysql;

use uuid::Uuid;

pub struct Order {
    pub id: Uuid,
    pub menu_item: String,
    pub cooking_time: String,
}

pub trait Storage: Send + Sync {
    fn get_table_orders(&self, table_id: u64) -> Result<Vec<Order>, String>;
    fn get_table_order(&self, table_id: u64, order_id: Uuid) -> Result<Order, String>;
    fn add_table_orders(&self, table_id: u64, orders: Vec<Order>) -> Result<Vec<Uuid>, String>;
    fn delete_table_order(&self, table_id: u64, order_id: Uuid) -> Result<(), String>;
}
