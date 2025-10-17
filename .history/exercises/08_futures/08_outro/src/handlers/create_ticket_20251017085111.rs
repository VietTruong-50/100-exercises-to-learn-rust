use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;
use uuid::Uuid;

use crate::models::ticket::{AppState, Ticket};

#[derive(Deserialize)]
struct CreateTicketInput {
    title: String,
    description: Option<String>,
}

pub(crate) async fn create_ticket(
    State(state): State<AppState>,
    Json(payload): Json<CreateTicketInput>,
) -> (StatusCode, Json<Ticket>) {
    let new_ticket = Ticket {
        id: Uuid::new_v4(),
        title: payload.title,
        description: payload.description,
        status: "open".into(),
    };

    let mut tickets = state.tickets.write().await;
    tickets.push(new_ticket.clone());

    (StatusCode::CREATED, Json(new_ticket))
}