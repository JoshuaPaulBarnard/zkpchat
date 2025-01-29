use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, delete};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Debug)]
struct StoreRequest {
    encrypted_data: String,
    destination_url: String,
}

struct AppState {
    messages: Mutex<Vec<String>>,
}

#[post("/store")]
async fn store_message(
    data: web::Data<AppState>,
    req: web::Json<StoreRequest>,
) -> impl Responder {
    let payload = req.into_inner();
    let mut messages = data.messages.lock().unwrap();
    messages.push(payload.encrypted_data);

    HttpResponse::Ok().body("Message stored successfully")
}

#[get("/messages")]
async fn get_messages(
    data: web::Data<AppState>,
) -> impl Responder {
    let messages = data.messages.lock().unwrap();
    let json = serde_json::to_string(&*messages).unwrap();
    HttpResponse::Ok().content_type("application/json").body(json)
}

#[get("/messages/{index}")]
async fn get_message_by_index(data: web::Data<AppState>, index: web::Path<usize>) -> impl Responder {
    let messages = data.messages.lock().unwrap();
    if *index < messages.len() {
        HttpResponse::Ok().json(&messages[*index])
    } else {
        HttpResponse::NotFound().body("Message not found")
    }
}

#[delete("/messages/{index}")]
async fn delete_message_by_index(data: web::Data<AppState>, index: web::Path<usize>) -> impl Responder {
    let mut messages = data.messages.lock().unwrap();
    if *index < messages.len() {
        messages.remove(*index);
        HttpResponse::Ok().body("Message deleted")
    } else {
        HttpResponse::NotFound().body("Message not found")
    }
}

#[delete("/messages")]
async fn delete_all_messages(data: web::Data<AppState>) -> impl Responder {
    let mut messages = data.messages.lock().unwrap();
    messages.clear();
    HttpResponse::Ok().body("All messages deleted")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        messages: Mutex::new(Vec::new()),
    });

    println!("Starting Storage Node on http://127.0.0.1:8082");

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(store_message)
            .service(get_messages)
            .service(get_message_by_index)
            .service(delete_message_by_index)
            .service(delete_all_messages)
    })
        .bind(("127.0.0.1", 8082))?
        .run()
        .await
}
