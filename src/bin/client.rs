use tokio;
use reqwest;
use futures::future::join_all;
use serde_json::json;
use uuid::Uuid;
use std::time::Duration;

#[tokio::main]
async fn main() {
    wait_for_server().await;

    simulate_clients().await;
}

async fn wait_for_server() {
    let client = reqwest::Client::new();
    let base_url = "http://localhost:8000";

    let mut attempts = 0;
    while attempts < 60 {
        match client.get(base_url).send().await {
            Ok(_) => {
                println!("Server is ready!");
                return;
            },
            Err(_) => {
                println!("Server not ready, waiting... (Attempt {})", attempts + 1);
                tokio::time::sleep(Duration::from_secs(1)).await;
                attempts += 1;
            }
        }
    }
    panic!("Server failed to start after 60 seconds");
}

async fn simulate_clients() {
    let client = reqwest::Client::new();
    let base_url = "http://localhost:8000";

    let mut tasks = vec![];

    for i in 1..=5 {
        let client = client.clone();
        let base_url = base_url.to_string();

        let task = tokio::spawn(async move {
            println!("Client {} started", i);

            let add_order_response = client.post(&format!("{}/tables/{}/orders", base_url, i))
                .json(&json!({
                    "orders": [
                        { "menu_item": "Pizza" },
                        { "menu_item": "Salad" }
                    ]
                }))
                .send()
                .await;

            match add_order_response {
                Ok(response) => {
                    println!("Client {} - Add orders status: {}", i, response.status());
                    if let Ok(order_ids) = response.json::<Vec<Uuid>>().await {
                        if let Ok(get_orders_response) = client.get(&format!("{}/tables/{}/orders", base_url, i))
                            .send()
                            .await
                        {
                            println!("Client {} - Get all orders status: {}", i, get_orders_response.status());
                        }

                        if let Some(order_id) = order_ids.first() {
                            if let Ok(get_order_response) = client.get(&format!("{}/tables/{}/orders/{}", base_url, i, order_id))
                                .send()
                                .await
                            {
                                println!("Client {} - Get specific order status: {}", i, get_order_response.status());
                            }
                        }

                        if let Some(order_id) = order_ids.last() {
                            if let Ok(delete_order_response) = client.delete(&format!("{}/tables/{}/orders/{}", base_url, i, order_id))
                                .send()
                                .await
                            {
                                println!("Client {} - Delete order status: {}", i, delete_order_response.status());
                            }
                        }
                    }
                },
                Err(e) => println!("Client {} - Error adding orders: {}", i, e),
            }

            println!("Client {} finished", i);
        });

        tasks.push(task);
    }

    join_all(tasks).await;
}