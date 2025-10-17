use crate::ticket::{TicketDraft, TicketId, TicketPatch};
use crate::store::{StoreError, TicketStore};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use tracing::{debug, error, info, warn};

// Application state
#[derive(Clone)]
pub struct AppState {
    pub store: TicketStore,
}

// Error handling with logging
impl IntoResponse for StoreError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            StoreError::NotFound(id) => {
                warn!("🔍 Ticket not found: {}", id);
                (StatusCode::NOT_FOUND, format!("Ticket {} not found", id))
            }
        };

        let body = Json(json!({
            "error": error_message,
            "timestamp": chrono::Utc::now().to_rfc3339(),
        }));

        error!("❌ API Error: {} - {}", status, error_message);
        (status, body).into_response()
    }
}

// Create a new ticket
pub async fn create_ticket(
    State(state): State<AppState>,
    Json(draft): Json<TicketDraft>,
) -> Result<impl IntoResponse, StoreError> {
    info!("📝 Creating new ticket: title='{}'", draft.title);
    debug!("Ticket draft details: {:?}", draft);

    let ticket_id = state.store.create_ticket(draft).await;
    info!("✅ Ticket created successfully with ID: {}", ticket_id);

    let ticket = state.store.get_ticket(ticket_id).await?;
    debug!("Created ticket details: {:?}", ticket);

    info!("📤 Returning created ticket");
    Ok((StatusCode::CREATED, Json(ticket)))
}

// Get a specific ticket
pub async fn get_ticket(
    State(state): State<AppState>,
    Path(ticket_id): Path<TicketId>,
) -> Result<impl IntoResponse, StoreError> {
    info!("🔍 Retrieving ticket with ID: {}", ticket_id);

    match state.store.get_ticket(ticket_id).await {
        Ok(ticket) => {
            info!("✅ Ticket found: {}", ticket_id);
            debug!("Ticket details: {:?}", ticket);
            Ok(Json(ticket))
        }
        Err(e) => {
            warn!("❌ Failed to retrieve ticket {}: {}", ticket_id, e);
            Err(e)
        }
    }
}

// Update a ticket
pub async fn update_ticket(
    State(state): State<AppState>,
    Path(ticket_id): Path<TicketId>,
    Json(patch): Json<TicketPatch>,
) -> Result<impl IntoResponse, StoreError> {
    info!("✏️  Updating ticket: {}", ticket_id);
    debug!("Update patch: {:?}", patch);

    match state.store.update_ticket(ticket_id, patch).await {
        Ok(ticket) => {
            info!("✅ Ticket updated successfully: {}", ticket_id);
            debug!("Updated ticket: {:?}", ticket);
            Ok(Json(ticket))
        }
        Err(e) => {
            warn!("❌ Failed to update ticket {}: {}", ticket_id, e);
            Err(e)
        }
    }
}

// List all tickets
pub async fn list_tickets(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, StoreError> {
    info!("📋 Retrieving all tickets");

    let tickets = state.store.list_tickets().await;
    let count = tickets.len();
    
    info!("✅ Retrieved {} tickets", count);
    debug!("Tickets: {:?}", tickets);

    Ok(Json(json!({
        "tickets": tickets,
        "count": count,
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}

// Health check endpoint
pub async fn health_check() -> impl IntoResponse {
    debug!("🏥 Health check requested");
    
    let response = json!({
        "status": "healthy",
        "service": "ticket-api",
        "version": "1.0.0",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "uptime": "N/A" // Could implement actual uptime tracking
    });

    info!("✅ Health check: OK");
    Json(response)
}