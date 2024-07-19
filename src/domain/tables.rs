use std::collections::HashMap;
use uuid::Uuid;
use rand::Rng;
use crate::protocol::protocol::{TableResponse, OrderResponse};
use crate::db::memdb::{OrderDB, TableDB, MemDb, OrdersInput};
use rocket::State;

pub fn get_all_tables(db: &State<MemDb>) -> Result<Vec<TableResponse>, String> {
    let tables = db.tables.lock().map_err(|_| "Failed get tables".to_string())?;
    let all_tables = tables.values().map(convert_to_table_response).collect();
    Ok(all_tables)
}

pub fn get_orders(table_id: u64, db: &State<MemDb>) -> Result<Vec<OrderResponse>, String> {
    let tables = db.tables.lock().map_err(|_| "Failed get tables".to_string())?;
    if let Some(table) = tables.get(&table_id) {
        Ok(table.orders.values().map(|order| convert_to_order_response(order)).collect())
    } else {
        Err("Table not found".to_string())
    }
}

pub fn get_order(table_id: u64, order_id: Uuid, db: &State<MemDb>) -> Result<OrderResponse, String> {
    let tables = db.tables.lock().map_err(|_| "Failed get tables".to_string())?;
    tables.get(&table_id)
        .and_then(|table| table.orders.get(&order_id))
        .map(convert_to_order_response)
        .ok_or_else(|| "Order not found".to_string())
}

pub fn add_orders(table_id: u64, orders_data: OrdersInput, db: &State<MemDb>) -> Result<Vec<Uuid>, String> {
    let mut tables = db.tables.lock().map_err(|_| "Failed to acquire tables lock".to_string())?;
    let mut order_ids = Vec::new();

    for order_input in &orders_data.orders {
        let cooking_time = format!("{} minutes", rand::thread_rng().gen_range(5..=15));
        let order_id = Uuid::new_v4();
        let order = OrderDB {
            order_id,
            menu_item: order_input.menu_item.clone(),
            cooking_time,
        };

        tables.entry(table_id).or_insert_with(|| TableDB {
            table_id,
            orders: HashMap::new(),
        }).orders.insert(order_id, order);

        order_ids.push(order_id);
    }
    Ok(order_ids)
}

pub fn remove_order(table_id: u64, order_id: Uuid, db: &State<MemDb>) -> Result<(), String> {
    let mut tables = db.tables.lock().map_err(|_| "Failed get tables".to_string())?;
    if let Some(table) = tables.get_mut(&table_id) {
        table.orders.remove(&order_id);
        Ok(())
    } else {
        Err("Table not found".to_string())
    }
}

fn convert_to_table_response(table: &TableDB) -> TableResponse {
    TableResponse {
        id: table.table_id,
        orders: table.orders.iter().map(|(id, order)| (*id, convert_to_order_response(order))).collect(),
    }
}

fn convert_to_order_response(order: &OrderDB) -> OrderResponse {
    OrderResponse {
        id: order.order_id,
        menu_item: order.menu_item.clone(),
        cooking_time: order.cooking_time.clone(),
    }
}