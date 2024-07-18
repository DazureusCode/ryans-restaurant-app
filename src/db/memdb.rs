use std::collections::HashMap;
use uuid::Uuid;
use std::sync::Mutex;

pub struct MemDb {
    pub tables: Mutex<HashMap<u64, TableDB>>,
}

impl MemDb {
    pub fn new() -> Self {
        Self {
            tables: Mutex::new(HashMap::new()),
        }
    }

    pub fn add_table(&self, table_id: u64) {
        let mut tables = self.tables.lock().unwrap();
        tables.insert(table_id, TableDB {
            table_id,
            orders: HashMap::new(),
        });
    }
}

pub struct TableDB {
    pub table_id: u64,
    pub orders: HashMap<Uuid, OrderDB>,
}

pub struct OrderDB {
    pub order_id: Uuid,
    pub menu_item: String,
    pub cooking_time: String,
}