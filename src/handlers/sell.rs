use crate::models::*;
use crate::state::AppState;
use actix_web::{HttpResponse, Responder, post, web};

#[post("/sell")]
pub async fn sell(state: web::Data<AppState>, req: web::Json<SellRequest>) -> impl Responder {
    let mut supply = req.volume;
    let mut bids_guard = state.state.bids.lock().unwrap();

    if !bids_guard.is_empty() {
        let mut allocations_guard = state.state.allocations.lock().unwrap();
        for (_price, queue) in bids_guard.iter_mut().rev() {
            while supply > 0 && !queue.is_empty() {
                let front = queue.front_mut().unwrap();

                let to_allocate = supply.min(front.volume);
                *allocations_guard.entry(front.username.clone()).or_insert(0) += to_allocate;

                front.volume -= to_allocate;
                supply -= to_allocate;

                if front.volume == 0 {
                    queue.pop_front();
                }
            }

            if supply == 0 {
                break;
            }
        }

        bids_guard.retain(|_, q| !q.is_empty());
    }

    drop(bids_guard);

    if supply > 0 {
        let mut supply_guard = state.state.supply.lock().unwrap();
        *supply_guard += supply;
    }

    HttpResponse::Ok()
}
