use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use reqwest::Client;

#[derive(Serialize, Deserialize, Debug)]
struct RelayRequest {
    /// The encrypted message payload (bytes in base64 or hex, for simplicity)
    encrypted_data: String,
    /// A destination storage node URL where this data should be forwarded
    destination_url: String,
}

#[post("/relay")]
async fn relay_message(
    data: web::Data<AppState>,
    req: web::Json<RelayRequest>,
) -> impl Responder {
    let client = data.http_client.clone();
    let payload = req.into_inner();

    // Forward the payload to the storage node
    let response = client
        .post(format!("{}/store", payload.destination_url))
        .json(&payload)
        .send()
        .await;

    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                HttpResponse::Ok().body("Relayed to storage node successfully")
            } else {
                HttpResponse::BadGateway().body("Failed to relay to storage node")
            }
        }
        Err(e) => {
            eprintln!("Relay error: {:?}", e);
            HttpResponse::InternalServerError().body("Relay node encountered an error")
        }
    }
}

struct AppState {
    http_client: Client,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Use `reqwest` for making HTTP requests
    let http_client = Client::new();

    let app_data = web::Data::new(AppState {
        http_client,
    });

    println!("Starting Relay Node on http://127.0.0.1:8081");

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(relay_message)
    })
        .bind(("127.0.0.1", 8081))?
        .run()
        .await
}
