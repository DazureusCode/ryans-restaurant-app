use rocket::{serde::json::Json, response::status, State, get, post, delete};
use crate::protocol::protocol::{TableResponse, OrderResponse};
use crate::db::mysql::OrdersInput;
use crate::api::Storage;
use uuid::Uuid;

#[get("/tables")]
pub fn get_tables(storage: &State<Box<dyn Storage + Send + Sync>>) -> Result<Json<Vec<TableResponse>>, status::Custom<String>> {
    storage.get_tables()
        .map(Json)
        .map_err(|e| status::Custom(rocket::http::Status::InternalServerError, e))
}

#[get("/tables/<table_id>/orders")]
pub fn get_table_orders(table_id: u64, storage: &State<Box<dyn Storage + Send + Sync>>) -> Result<Json<Vec<OrderResponse>>, status::Custom<String>> {
    storage.get_table_orders(table_id)
        .map(Json)
        .map_err(|e| status::Custom(rocket::http::Status::InternalServerError, e))
}

#[get("/tables/<table_id>/orders/<order_id>")]
pub fn get_table_order(table_id: u64, order_id: String, storage: &State<Box<dyn Storage + Send + Sync>>) -> Result<Json<OrderResponse>, status::Custom<String>> {
    let uuid = Uuid::parse_str(&order_id).map_err(|e| status::Custom(rocket::http::Status::BadRequest, e.to_string()))?;
    storage.get_table_order(table_id, uuid)
        .map(Json)
        .map_err(|e| status::Custom(rocket::http::Status::InternalServerError, e))
}

#[post("/tables/<table_id>/orders", data = "<orders_data>")]
pub fn add_table_orders(table_id: u64, orders_data: Json<OrdersInput>, storage: &State<Box<dyn Storage + Send + Sync>>) -> Result<Json<Vec<Uuid>>, status::Custom<String>> {
    storage.add_table_orders(table_id, orders_data.into_inner())
        .map(Json)
        .map_err(|e| status::Custom(rocket::http::Status::InternalServerError, e))
}

#[delete("/tables/<table_id>/orders/<order_id>")]
pub fn delete_table_order(table_id: u64, order_id: String, storage: &State<Box<dyn Storage + Send + Sync>>) -> Result<status::NoContent, status::Custom<String>> {
    let uuid = Uuid::parse_str(&order_id).map_err(|e| status::Custom(rocket::http::Status::BadRequest, e.to_string()))?;
    storage.delete_table_order(table_id, uuid)
        .map(|_| status::NoContent)
        .map_err(|e| status::Custom(rocket::http::Status::InternalServerError, e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::http::Status;
    use rocket::local::blocking::Client;
    use rocket::figment::Figment;
    use rocket::Config;
    use crate::db::mysql::{OrdersInput, OrderInput};
    use uuid::Uuid;
    use mysql::serde_json;

    struct MockStorage;

    impl Storage for MockStorage {
        fn get_tables(&self) -> Result<Vec<TableResponse>, String> {
            Ok(vec![
                TableResponse {
                    id: 1,
                    orders: vec![(Uuid::new_v4(), OrderResponse {
                        id: Uuid::new_v4(),
                        menu_item: "Mock Item".to_string(),
                        cooking_time: "10 minutes".to_string(),
                    })].into_iter().collect(),
                }
            ])
        }

        fn get_table_orders(&self, _table_id: u64) -> Result<Vec<OrderResponse>, String> {
            Ok(vec![
                OrderResponse {
                    id: Uuid::new_v4(),
                    menu_item: "Mock Item".to_string(),
                    cooking_time: "10 minutes".to_string(),
                }
            ])
        }

        fn get_table_order(&self, _table_id: u64, _order_id: Uuid) -> Result<OrderResponse, String> {
            Ok(OrderResponse {
                id: Uuid::new_v4(),
                menu_item: "Mock Item".to_string(),
                cooking_time: "10 minutes".to_string(),
            })
        }

        fn add_table_orders(&self, _table_id: u64, _orders: OrdersInput) -> Result<Vec<Uuid>, String> {
            Ok(vec![Uuid::new_v4()])
        }

        fn delete_table_order(&self, _table_id: u64, _order_id: Uuid) -> Result<(), String> {
            Ok(())
        }
    }

    fn setup_rocket() -> rocket::Rocket<rocket::Build> {
        let figment = Figment::from(Config::default())
            .merge(("secret_key", "a".repeat(64)))
            .merge(("databases.mysql.url", "mysql://root:password@localhost:3306/restaurant_test"));

        rocket::custom(figment)
            .manage(Box::new(MockStorage) as Box<dyn Storage + Send + Sync>)
            .mount("/", routes![
                get_tables,
                get_table_orders,
                get_table_order,
                add_table_orders,
                delete_table_order,
            ])
    }

    #[test]
    fn test_get_tables() {
        let client = Client::tracked(setup_rocket()).expect("valid rocket instance");
        let response = client.get("/tables").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_add_table_orders() {
        let client = Client::tracked(setup_rocket()).expect("valid rocket instance");

        let orders_input = OrdersInput {
            orders: vec![OrderInput { menu_item: "Pizza".into() }],
        };

        let response = client.post("/tables/1/orders")
            .header(rocket::http::ContentType::JSON)
            .body(serde_json::to_string(&orders_input).unwrap())
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_delete_table_order() {
        let client = Client::tracked(setup_rocket()).expect("valid rocket instance");
        let order_id = Uuid::new_v4().to_string();
        let response = client.delete(format!("/tables/1/orders/{}", order_id)).dispatch();
        assert_eq!(response.status(), Status::NoContent);
    }

    #[test]
    fn test_get_table_orders() {
        let client = Client::tracked(setup_rocket()).expect("valid rocket instance");
        let response = client.get("/tables/1/orders").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_get_table_order() {
        let client = Client::tracked(setup_rocket()).expect("valid rocket instance");
        let order_id = Uuid::new_v4().to_string();
        let response = client.get(format!("/tables/1/orders/{}", order_id)).dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}
