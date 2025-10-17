use std::sync::{Arc, RwLock};

use axum::Router;
use outro_08::models::ticket::AppState;

#[tokio::main]
async fn main () {
    let state = AppState {
        tickets: Arc::new(RwLock::new(Vec::new())),
    };

    let app = Router::new()
    .route("/tickets", method_router)
}