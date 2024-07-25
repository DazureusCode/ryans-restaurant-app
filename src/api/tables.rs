use crate::domain::tables::{add_orders, get_order, get_orders, remove_order};
use crate::protocol::protocol::{OrderResponse, OrdersInput};
use crate::ServerState;
use rocket::{delete, get, post, response::status, serde::json::Json, State};
use uuid::Uuid;

#[get("/tables/<table_id>/orders")]
pub fn get_table_orders(
    table_id: u64,
    state: &State<Box<ServerState>>,
) -> Result<Json<Vec<OrderResponse>>, status::Custom<String>> {
    let mut order_results = Vec::new();
    get_orders(table_id, state)
        .and_then(|domain_orders| {
            for order_input in domain_orders {
                let order = OrderResponse {
                    id: order_input.id,
                    menu_item: order_input.menu_item,
                    cooking_time: order_input.cooking_time,
                };
                order_results.push(order);
            }
            Ok(order_results)
        })
        .map(Json)
        .map_err(|e| status::Custom(rocket::http::Status::InternalServerError, e))
}

#[get("/tables/<table_id>/orders/<order_id>")]
pub fn get_table_order(
    table_id: u64,
    order_id: String,
    state: &State<Box<ServerState>>,
) -> Result<Json<OrderResponse>, status::Custom<String>> {
    let order_id = Uuid::parse_str(&order_id)
        .map_err(|e| status::Custom(rocket::http::Status::BadRequest, e.to_string()))?;
    get_order(table_id, order_id, state)
        .map(|order| {
            let order_input = OrderResponse {
                id: order.id,
                menu_item: order.menu_item,
                cooking_time: order.cooking_time,
            };
            Json(order_input)
        })
        .map_err(|e| status::Custom(rocket::http::Status::InternalServerError, e))
}

#[post("/tables/<table_id>/orders", data = "<orders_data>")]
pub fn add_table_orders(
    table_id: u64,
    orders_data: Json<OrdersInput>,
    state: &State<Box<ServerState>>,
) -> Result<Json<Vec<Uuid>>, status::Custom<String>> {
    add_orders(table_id, orders_data.into_inner(), state)
        .map(Json)
        .map_err(|e| status::Custom(rocket::http::Status::InternalServerError, e))
}

#[delete("/tables/<table_id>/orders/<order_id>")]
pub fn delete_table_order(
    table_id: u64,
    order_id: String,
    state: &State<Box<ServerState>>,
) -> Result<Json<()>, status::Custom<String>> {
    let uuid = Uuid::parse_str(&order_id)
        .map_err(|e| status::Custom(rocket::http::Status::BadRequest, e.to_string()))?;
    remove_order(table_id, uuid, state)
        .map(|_| Json(()))
        .map_err(|e| status::Custom(rocket::http::Status::InternalServerError, e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use mysql::serde_json;
    use rocket::figment::Figment;
    use rocket::http::Status;
    use rocket::local::blocking::Client;
    use rocket::{routes, Build, Rocket};
    use uuid::Uuid;
    use crate::db::{Storage, Order};
    use crate::protocol::protocol::{OrderInput, OrdersInput};
    use crate::ServerState;

    struct MockStorage;

    impl Storage for MockStorage {
        fn get_table_orders(&self, _table_id: u64) -> Result<Vec<Order>, String> {
            Ok(vec![Order {
                id: Uuid::new_v4(),
                menu_item: "Mock Item".to_string(),
                cooking_time: "10 minutes".to_string(),
            }])
        }

        fn get_table_order(
            &self,
            _table_id: u64,
            _order_id: Uuid,
        ) -> Result<Order, String> {
            Ok(Order {
                id: Uuid::new_v4(),
                menu_item: "Mock Item".to_string(),
                cooking_time: "10 minutes".to_string(),
            })
        }

        fn add_table_orders(
            &self,
            _table_id: u64,
            _orders: Vec<Order>,
        ) -> Result<Vec<Uuid>, String> {
            Ok(vec![Uuid::new_v4()])
        }

        fn delete_table_order(&self, _table_id: u64, _order_id: Uuid) -> Result<(), String> {
            Ok(())
        }
    }

    fn setup_rocket() -> Rocket<Build> {
        let figment = Figment::from(rocket::Config::default())
            .merge(("secret_key", "a".repeat(64)))
            .merge(("databases.mysql.url", "mysql://root:password@localhost:3306/restaurant_test"));

        rocket::custom(figment)
            .manage(Box::new(ServerState {
                db: Box::new(MockStorage) as Box<dyn Storage + Send + Sync>,
            }))
            .mount("/", routes![
                get_table_orders,
                get_table_order,
                add_table_orders,
                delete_table_order,
            ])
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

    #[test]
    fn test_add_table_orders() {
        let client = Client::tracked(setup_rocket()).expect("valid rocket instance");

        let orders_input = OrdersInput {
            orders: vec![OrderInput {
                menu_item: "Pizza".into(),
            }],
        };

        let response = client
            .post("/tables/1/orders")
            .header(rocket::http::ContentType::JSON)
            .body(serde_json::to_string(&orders_input).unwrap())
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_delete_table_order() {
        let client = Client::tracked(setup_rocket()).expect("valid rocket instance");
        let order_id = Uuid::new_v4().to_string();
        let response = client
            .delete(format!("/tables/1/orders/{}", order_id))
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}
