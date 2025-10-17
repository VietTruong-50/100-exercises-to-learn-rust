use crate::data::{TicketDraft, TicketId, TicketPatch};
use crate::store::{StoreError, TicketStore};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

// Application state
#[derive(Clone)]
pub struct AppState {
    pub store: TicketStore,
}

// Error handling
impl IntoResponse for StoreError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            StoreError::NotFound(id) => {
                (StatusCode::NOT_FOUND, format!("Ticket {} not found", id))
            }
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

// Create a new ticket
pub async fn create_ticket(
    State(state): State<AppState>,
    Json(draft): Json<TicketDraft>,
) -> Result<impl IntoResponse, StoreError> {
    let ticket_id = state.store.create_ticket(draft).await;
    let ticket = state.store.get_ticket(ticket_id).await?;
    
    Ok((StatusCode::CREATED, Json(ticket)))
}

// Get a specific ticket
pub async fn get_ticket(
    State(state): State<AppState>,
    Path(ticket_id): Path<TicketId>,
) -> Result<impl IntoResponse, StoreError> {
    let ticket = state.store.get_ticket(ticket_id).await?;
    Ok(Json(ticket))
}

// Update a ticket
pub async fn update_ticket(
    State(state): State<AppState>,
    Path(ticket_id): Path<TicketId>,
    Json(patch): Json<TicketPatch>,
) -> Result<impl IntoResponse, StoreError> {
    let ticket = state.store.update_ticket(ticket_id, patch).await?;
    Ok(Json(ticket))
}

// List all tickets
pub async fn list_tickets(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, StoreError> {
    let tickets = state.store.list_tickets().await;
    Ok(Json(tickets))
}

// Health check endpoint
pub async fn health_check() -> impl IntoResponse {
    Json(json!({
        "status": "healthy",
        "service": "ticket-api"
    }))
}