
mod handlers;
mod store;

use axum::{
    routing::{get, patch, post},
    Router,
};
use handlers::{create_ticket, get_ticket, health_check, list_tickets, update_ticket, AppState};
use store::TicketStore;
use tower_http::cors::CorsLayer;
use tracing_subscriber;

#[tokio::main]
async fn main() {


    // Create the ticket store
    let store = TicketStore::new();
    let app_state = AppState { store };

    // Build the application with routes
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/tickets", post(create_ticket))
        .route("/tickets", get(list_tickets))
        .route("/tickets/:id", get(get_ticket))
        .route("/tickets/:id", patch(update_ticket))
        .layer(CorsLayer::permissive())
        .with_state(app_state);

    // Start the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 Server running on http://localhost:3000");
    
    axum::serve(listener, app).await.unwrap();
}