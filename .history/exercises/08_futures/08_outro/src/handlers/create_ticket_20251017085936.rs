use axum::{extract::State, Json};
use serde::Deserialize;
use serde_json::json;
use crate::models::ticket::{AppState, Ticket};


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