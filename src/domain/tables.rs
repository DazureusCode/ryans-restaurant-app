use std::collections::HashMap;
use uuid::Uuid;
use rand::Rng;
use rocket::State;
use crate::protocol::protocol::OrdersInput;
use crate::ServerState;
use crate::db::Order as DBOrder;

pub struct Order {
    pub id: Uuid,
    pub menu_item: String,
    pub cooking_time: String,
}

pub struct Table {
    pub id: u64,
    pub orders: HashMap<Uuid, crate::db::Order>,
}

pub fn get_orders(table_id: u64, state: &State<Box<ServerState>>) -> Result<Vec<Order>, String> {
    let mut order_results = Vec::new();
    state.db.get_table_orders(table_id)
    .and_then(|orders| {
        for order_input in orders {
            let order = Order {
                id: order_input.id,
                menu_item: order_input.menu_item,
                cooking_time: order_input.cooking_time,
            };
            order_results.push(order);
        }
        Ok(order_results)
    })
}

pub fn get_order(table_id: u64, order_id: Uuid, state: &State<Box<ServerState>>) -> Result<Order, String> {
    state.db.get_table_order(table_id, order_id)
        .map(|order| {
            Order {
                id: order.id,
                menu_item: order.menu_item,
                cooking_time: order.cooking_time,
            }
        })
}

pub fn add_orders(table_id: u64, orders_data: OrdersInput, state: &State<Box<ServerState>>) -> Result<Vec<Uuid>, String> {
    let orders = orders_data.orders;
    let mut domain_orders = Vec::new();
    for order_input in orders {
        let cooking_time = format!("{} minutes", rand::thread_rng().gen_range(5..=15));
        let id = Uuid::new_v4();
        let order = Order {
            id,
            menu_item: order_input.menu_item,
            cooking_time,
        };
        domain_orders.push(order);
    }
    let mut db_orders = Vec::new();
    for order_input in domain_orders {
        let order = DBOrder {
            id: order_input.id,
            menu_item: order_input.menu_item,
            cooking_time: order_input.cooking_time,
        };
        db_orders.push(order);
    }
    state.db.add_table_orders(table_id, db_orders)
}

pub fn remove_order(table_id: u64, order_id: Uuid, state: &State<Box<ServerState>>) -> Result<(), String> {
    state.db.delete_table_order(table_id, order_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::mysql::{OrderInput, MySqlDb};
    use dotenv::dotenv;
    use mysql::params;
    use uuid::Uuid;
    use std::env;

    fn setup_test_db() -> MySqlDb {
        dotenv().ok();
        let database_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be established");
        MySqlDb::new(&database_url)
    }

    fn get_connection() -> PooledConn {
        let db = setup_test_db();
        db.pool.get_conn().expect("Failed to get connection")
    }

    const INSERT_ORDER_SQL: &str = "INSERT INTO orders (order_id, table_id, menu_item, cooking_time) VALUES (:order_id, :table_id, 'Mock Item', '10 minutes')";
    const DELETE_ORDER_SQL: &str = "DELETE FROM orders WHERE order_id = :order_id";

    #[test]
    fn test_get_orders() {
        let mut conn = get_connection();
        let table_id = 1;
        let order_id = Uuid::new_v4();

        conn.exec_drop(
            INSERT_ORDER_SQL,
            params! {
                "order_id" => order_id.to_string(),
                "table_id" => table_id,
            }
        ).unwrap();
        let result = get_orders(table_id, &mut conn);
        assert!(result.is_ok(), "Expected Ok but got Err: {:?}", result.err());
        conn.exec_drop(
            DELETE_ORDER_SQL,
            params! {
                "order_id" => order_id.to_string(),
            }
        ).unwrap();
    }

    #[test]
    fn test_get_order() {
        let mut conn = get_connection();
        let table_id = 1;
        let order_id = Uuid::new_v4();

        conn.exec_drop(
            INSERT_ORDER_SQL,
            params! {
                "order_id" => order_id.to_string(),
                "table_id" => table_id,
            }
        ).unwrap();
        let result = get_order(table_id, order_id, &mut conn);
        assert!(result.is_ok(), "Expected Ok but got Err: {:?}", result.err());
        conn.exec_drop(
            DELETE_ORDER_SQL,
            params! {
                "order_id" => order_id.to_string(),
            }
        ).unwrap();
    }

    #[test]
    fn test_add_orders() {
        let mut conn = get_connection();
        let orders_input = OrdersInput {
            orders: vec![OrderInput { menu_item: "Mock Item".into() }],
        };
        assert!(add_orders(1, orders_input, &mut conn).is_ok());
    }

    #[test]
    fn test_remove_order() {
        let mut conn = get_connection();
        let table_id = 1;
        let order_id = Uuid::new_v4();
        conn.exec_drop(
            INSERT_ORDER_SQL,
            params! {
                "order_id" => order_id.to_string(),
                "table_id" => table_id,
            }
        ).unwrap();
        let result = remove_order(table_id, order_id, &mut conn);
        assert!(result.is_ok(), "Expected Ok but got Err: {:?}", result.err());
    }
}
