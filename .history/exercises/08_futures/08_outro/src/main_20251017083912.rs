use std::{os::unix::net::SocketAddr, sync::{Arc, RwLock}};

use axum::{routing::{get, post}, Router};
use outro_08::{handlers::{create_ticket, get_ticket, patch_ticket}, models::ticket::AppState};

#[tokio::main]
async fn main () {
    let state = AppState {
        tickets: Arc::new(RwLock::new(Vec::new())),
    };

    let app = Router::new()
    .route("/tickets", post(create_ticket))
    .route("/tickets/:id", get(get_ticket).patch(patch_ticket))
    .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 0], 8080));
    println!("Listening on http://{addr}");
    axum::serve(Tokio::new(app), addr).await.unwrap();
}