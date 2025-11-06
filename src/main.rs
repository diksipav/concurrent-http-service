use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
struct BuyRequest {
    username: String,
    volume: u64,
    price: u64,
}

#[derive(Deserialize)]
struct SellRequest {
    volume: u64,
}

struct Bid {
    username: String,
    price: u64,
    volume: u64,
    seq: u64,
}

#[derive(Clone, Default)]
struct AppState;

#[post("/buy")]
async fn buy(state: web::Data<AppState>, req: web::Json<BuyRequest>) -> impl Responder {
    // TODO
    HttpResponse::Ok()
}

#[post("/sell")]
async fn sell(state: web::Data<AppState>, req: web::Json<SellRequest>) -> impl Responder {
    // TODO
    HttpResponse::Ok()
}

#[get("/allocation")]
async fn allocation(state: web::Data<AppState>) -> impl Responder {
    // TODO
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server...");
    let state = AppState::default();
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(buy)
            .service(sell)
            .service(allocation)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
