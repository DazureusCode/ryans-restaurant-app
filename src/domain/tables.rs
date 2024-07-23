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
