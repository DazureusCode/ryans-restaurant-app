#[macro_use] extern crate rocket;

mod api;
mod db;
mod domain;
mod protocol;

use db::mysql::MySqlDb;
use dotenv::dotenv;
use std::env;

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    let database_url = env::var("RESTAURANT_DATABASE_URL").expect("RESTAURANT_DATABASE_URL must be declared");
    let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be declared");

    let db = MySqlDb::new(&database_url);

    rocket::custom(
        rocket::Config::figment()
            .merge(("databases", rocket::figment::value::Dict::from_iter(
                std::iter::once((String::from("mysql"), rocket::figment::value::Value::from(database_url)))
            )))
            .merge(("secret_key", secret_key))
    )
        .manage(db)
        .mount("/", routes![
        api::tables::get_tables,
        api::tables::add_table_orders,
        api::tables::delete_table_order,
        api::tables::get_table_orders,
        api::tables::get_table_order,
    ])
}