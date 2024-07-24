use mysql::prelude::*;
use mysql::*;
use rocket::serde::{Deserialize, Serialize};
use crate::protocol::protocol::{TableResponse, OrderResponse};
use crate::domain::tables::{get_all_tables, get_orders, get_order, add_orders, remove_order};
use crate::api::Storage;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderInput {
    pub menu_item: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OrdersInput {
    pub orders: Vec<OrderInput>,
}

pub struct MySqlDb {
    pub pool: Pool,
}

impl MySqlDb {
    pub fn new(database_url: &str) -> Self {
        let opts = Opts::from_url(database_url).expect("Incorrect database URL");
        let pool = Pool::new(opts).expect("Failed to create MySQL connection pool.");

        let mut conn = pool.get_conn().expect("Failed to establish connection.");
        for table_id in 1..=100 {
            conn.exec_drop(
                "INSERT IGNORE INTO tables (table_id) VALUES (:table_id)",
                params! {
                    "table_id" => table_id,
                }
            ).expect("Failed to insert table");
        }

        MySqlDb { pool }
    }
}

impl Storage for MySqlDb {
    fn get_tables(&self) -> Result<Vec<TableResponse>, String> {
        let mut conn = self.pool.get_conn().map_err(|e| e.to_string())?;
        get_all_tables(&mut conn)
    }

    fn get_table_orders(&self, table_id: u64) -> Result<Vec<OrderResponse>, String> {
        let mut conn = self.pool.get_conn().map_err(|e| e.to_string())?;
        get_orders(table_id, &mut conn)
    }

    fn get_table_order(&self, table_id: u64, order_id: Uuid) -> Result<OrderResponse, String> {
        let mut conn = self.pool.get_conn().map_err(|e| e.to_string())?;
        get_order(table_id, order_id, &mut conn)
    }

    fn add_table_orders(&self, table_id: u64, orders: OrdersInput) -> Result<Vec<Uuid>, String> {
        let mut conn = self.pool.get_conn().map_err(|e| e.to_string())?;
        add_orders(table_id, orders, &mut conn)
    }

    fn delete_table_order(&self, table_id: u64, order_id: Uuid) -> Result<(), String> {
        let mut conn = self.pool.get_conn().map_err(|e| e.to_string())?;
        remove_order(table_id, order_id, &mut conn)
    }
}