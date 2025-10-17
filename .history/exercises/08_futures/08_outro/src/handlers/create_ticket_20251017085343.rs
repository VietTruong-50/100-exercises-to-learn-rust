use axum::{extract::State, Json};
use serde_json::json;
use crate::models::ticket::{AppState, Ticket};

use crate::models::ticket::{AppState, Ticket};

#[derive(Deserialize)]
struct CreateTicketInput {
    title: String,
    description: Option<String>,
}

pub async fn create_ticket(
    State(state): State<AppState>,
    Json(ticket): Json<Ticket>,
) -> Json<serde_json::Value> {
    let mut tickets = state.tickets.write().await;
    tickets.push(ticket.clone());

    Json(json!({
        "status": "ok",
        "ticket": ticket
    }))
}