use super::models::{DecrementData, IncrementData};
use crate::AppState;
use actix_web::{post, web, HttpResponse, Responder};

#[post("/counter/inc")]
async fn increment(
    data: web::Data<AppState>,
    req_body: web::Json<IncrementData>,
) -> impl Responder {
    let mut current_counter = data.count.lock().unwrap();
    current_counter.count += req_body.times;
    println!("Counter: {}", current_counter.count);
    HttpResponse::Ok().json(current_counter.clone())
}

#[post("/counter/dec")]
async fn decrement(
    data: web::Data<AppState>,
    req_body: web::Json<DecrementData>,
) -> impl Responder {
    let mut current_counter = data.count.lock().unwrap();
    current_counter.count -= req_body.times;
    println!("Counter: {}", current_counter.count);
    HttpResponse::Ok().json(current_counter.clone())
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(increment).service(decrement);
}
