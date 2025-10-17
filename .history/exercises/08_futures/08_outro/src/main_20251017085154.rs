use std::{ net::SocketAddr, sync::Arc};

use axum::{routing::{get, post}, Router};
use outro_08::{ handlers::create_ticket, models::ticket::AppState};
use tokio::{sync::RwLock};

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
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
