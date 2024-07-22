use mysql::prelude::*;
use mysql::*;
use rocket::serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct OrderInput {
    pub menu_item: String,
}

#[derive(Debug, Deserialize)]
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