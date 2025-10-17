use std::sync::{Arc, RwLock};

use outro_08::models::ticket::AppState;

#[tokio::main]
async fn main () {
    let state = AppState {
        tickets: Arc::new(RwLock::new(Vec::new())),
    };

    lt
}