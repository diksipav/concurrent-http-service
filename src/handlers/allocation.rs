use crate::models::*;
use crate::state::AppState;
use actix_web::{HttpResponse, Responder, get, web};

#[get("/allocation")]
pub async fn allocation(
    state: web::Data<AppState>,
    query: web::Query<AllocationQuery>,
) -> impl Responder {
    let Some(username) = &query.username else {
        return HttpResponse::BadRequest().body("missing 'username' query parameter");
    };

    let allocations_guard = state.state.allocations.lock().unwrap();
    println!("didi {:?}", allocations_guard);
    match allocations_guard.get(username) {
        Some(allocation) => HttpResponse::Ok()
            .content_type("text/plain")
            .body(allocation.to_string()),
        None => HttpResponse::NotFound().body(format!("username '{}' not found", username)),
    }
}
