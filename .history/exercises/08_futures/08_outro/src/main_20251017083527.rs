use std::sync::{Arc, RwLock};

use axum::{routing::post, Router};
use outro_08::{handlers::create_ticket, models::ticket::AppState};

#[tokio::main]
async fn main () {
    let state = AppState {
        tickets: Arc::new(RwLock::new(Vec::new())),
    };

    let app = Router::new()
    .route("/tickets", post(create_ticket))
    .route("/tickets/:id", method_router)
}