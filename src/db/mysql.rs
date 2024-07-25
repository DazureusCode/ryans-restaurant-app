use crate::db::{Order, Storage};
use mysql::prelude::*;
use mysql::*;
use uuid::Uuid;

pub struct MySqlDb {
    pub pool: Pool,
}

impl MySqlDb {
    pub fn new(database_url: &str) -> Self {
        let opts = Opts::from_url(database_url).expect("Incorrect database URL");
        let pool = Pool::new(opts).expect("Failed to create MySQL connection pool.");

        let conn = pool.get_conn().expect("Failed to establish connection.");
        Self::setup(conn).expect("Failed to create tables");

        MySqlDb { pool }
    }

    pub fn setup(mut conn: PooledConn) -> Result<(), String> {
        conn.exec_drop(
            "CREATE TABLE IF NOT EXISTS orders (
            id VARCHAR(255),
            menu_item VARCHAR(255),
            cooking_time VARCHAR(255),
            table_id INT
        )",
            (),
        )
        .map_err(|e| e.to_string())
    }
}

impl Storage for MySqlDb {
    fn get_table_orders(&self, table_id: u64) -> Result<Vec<Order>, String> {
        let mut conn = self.pool.get_conn().map_err(|e| e.to_string())?;
        conn.exec_map(
            "SELECT order_id, menu_item, cooking_time FROM orders WHERE table_id = :table_id",
            params! {
                "table_id" => table_id,
            },
            |(order_id, menu_item, cooking_time): (String, String, String)| Order {
                id: Uuid::parse_str(&order_id).unwrap(),
                menu_item,
                cooking_time,
            },
        )
        .map_err(|e| e.to_string())
    }

    fn get_table_order(&self, table_id: u64, order_id: Uuid) -> Result<Order, String> {
        let mut conn = self.pool.get_conn().map_err(|e| e.to_string())?;
        conn.exec_first(
            "SELECT order_id, menu_item, cooking_time FROM orders WHERE table_id = :table_id AND order_id = :order_id",
            params! {
            "table_id" => table_id,
            "order_id" => order_id.to_string(),
        }
        ).map(|row| {
            row.map(|(order_id, menu_item, cooking_time): (String, String, String)| Order {
                id: Uuid::parse_str(&order_id).unwrap(),
                menu_item,
                cooking_time,
            })
        }).map_err(|e| e.to_string()).and_then(|opt| opt.ok_or("Order not found".to_string()))
    }

    fn add_table_orders(&self, table_id: u64, orders: Vec<Order>) -> Result<Vec<Uuid>, String> {
        let mut conn = self.pool.get_conn().map_err(|e| e.to_string())?;
        let mut order_ids = Vec::new();

        for order_input in orders {
            conn.exec_drop(
                "INSERT INTO orders (order_id, table_id, menu_item, cooking_time) VALUES (:order_id, :table_id, :menu_item, :cooking_time)",
                params! {
                "order_id" => order_input.id.to_string(),
                "table_id" => table_id,
                "menu_item" => order_input.menu_item,
                "cooking_time" => order_input.cooking_time,
            }
            ).map_err(|e| e.to_string())?;
            order_ids.push(order_input.id);
        }
        Ok(order_ids)
    }

    fn delete_table_order(&self, table_id: u64, order_id: Uuid) -> Result<(), String> {
        let mut conn = self.pool.get_conn().map_err(|e| e.to_string())?;
        conn.exec_drop(
            "DELETE FROM orders WHERE table_id = :table_id AND order_id = :order_id",
            params! {
                "table_id" => table_id,
                "order_id" => order_id.to_string(),
            },
        )
        .map_err(|e| e.to_string())?;
        Ok(())
    }
}
