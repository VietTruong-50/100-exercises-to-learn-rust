use serde::Deserialize;

#[derive(Deserialize)]
struct PatchTicketInput {
    status: Option<String>,
    description: Option<String>,
}

async fn patch_ticket(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    Json(payload): Json<PatchTicketInput>,
) -> Result<Json<Ticket>, StatusCode> {
    let mut tickets = state.tickets.write().await;
    if let Some(ticket) = tickets.iter_mut().find(|t| t.id == id) {
        if let Some(status) = payload.status {
            ticket.status = status;
        }
        if let Some(desc) = payload.description {
            ticket.description = Some(desc);
        }
        return Ok(Json(ticket.clone()));
    }
    Err(StatusCode::NOT_FOUND)
}