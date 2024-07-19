#[macro_use] extern crate rocket;

mod api;
mod db;
mod domain;
mod protocol;

use db::memdb::MemDb;

#[launch]
fn rocket() -> _ {
    let db = MemDb::new();

    for table_id in 1..=100 {
        db.add_table(table_id);
    }
    
    rocket::build()
        .manage(db)
        .mount("/", routes![
            api::tables::get_tables,
            api::tables::get_table_order,
            api::tables::get_table_orders,
            api::tables::add_table_orders,
            api::tables::delete_table_order,
        ])
}