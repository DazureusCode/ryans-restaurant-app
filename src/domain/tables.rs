use uuid::Uuid;
use rand::Rng;
use crate::protocol::protocol::{TableResponse, OrderResponse};
use crate::db::mysql::OrdersInput;
use mysql::prelude::*;
use mysql::*;

pub fn get_all_tables(conn: &mut PooledConn) -> Result<Vec<TableResponse>, String> {
    let table_ids: Vec<u64> = conn.query("SELECT table_id FROM tables")
        .map_err(|e| e.to_string())?;

    let mut result = Vec::new();
    for table_id in table_ids {
        let orders = get_orders_for_table(conn, table_id)?;
        result.push(convert_to_table_response(table_id, orders));
    }

    Ok(result)
}

fn get_orders_for_table(conn: &mut PooledConn, table_id: u64) -> Result<Vec<OrderResponse>, String> {
    conn.exec_map(
        "SELECT order_id, menu_item, cooking_time FROM orders WHERE table_id = :table_id",
        params! {
            "table_id" => table_id,
        },
        |(order_id, menu_item, cooking_time): (String, String, String)| {
            OrderResponse {
                id: Uuid::parse_str(&order_id).unwrap(),
                menu_item,
                cooking_time,
            }
        }
    ).map_err(|e| e.to_string())
}

pub fn get_orders(table_id: u64, conn: &mut PooledConn) -> Result<Vec<OrderResponse>, String> {
    get_orders_for_table(conn, table_id)
}

pub fn get_order(table_id: u64, order_id: Uuid, conn: &mut PooledConn) -> Result<OrderResponse, String> {
    conn.exec_first(
        "SELECT order_id, menu_item, cooking_time FROM orders WHERE table_id = :table_id AND order_id = :order_id",
        params! {
            "table_id" => table_id,
            "order_id" => order_id.to_string(),
        }
    ).map(|row| {
        row.map(|(order_id, menu_item, cooking_time): (String, String, String)| OrderResponse {
            id: Uuid::parse_str(&order_id).unwrap(),
            menu_item,
            cooking_time,
        })
    }).map_err(|e| e.to_string()).and_then(|opt| opt.ok_or("Order not found".to_string()))
}

pub fn add_orders(table_id: u64, orders_data: OrdersInput, conn: &mut PooledConn) -> Result<Vec<Uuid>, String> {
    let mut order_ids = Vec::new();

    for order_input in orders_data.orders {
        let cooking_time = format!("{} minutes", rand::thread_rng().gen_range(5..=15));
        let order_id = Uuid::new_v4();
        conn.exec_drop(
            "INSERT INTO orders (order_id, table_id, menu_item, cooking_time) VALUES (:order_id, :table_id, :menu_item, :cooking_time)",
            params! {
                "order_id" => order_id.to_string(),
                "table_id" => table_id,
                "menu_item" => order_input.menu_item,
                "cooking_time" => cooking_time,
            }
        ).map_err(|e| e.to_string())?;
        order_ids.push(order_id);
    }
    Ok(order_ids)
}

pub fn remove_order(table_id: u64, order_id: Uuid, conn: &mut PooledConn) -> Result<(), String> {
    conn.exec_drop(
        "DELETE FROM orders WHERE table_id = :table_id AND order_id = :order_id",
        params! {
            "table_id" => table_id,
            "order_id" => order_id.to_string(),
        }
    ).map_err(|e| e.to_string())?;
    Ok(())
}

fn convert_to_table_response(table_id: u64, orders: Vec<OrderResponse>) -> TableResponse {
    TableResponse {
        id: table_id,
        orders: orders.into_iter().map(|order| (order.id, order)).collect(),
    }
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
    fn test_get_all_tables() {
        let mut conn = get_connection();
        assert!(get_all_tables(&mut conn).is_ok());
    }

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
