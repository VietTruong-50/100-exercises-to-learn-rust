use crate::models::ticket::AppState;

async fn get_ticket(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<Ticket>, StatusCode> {
    let tickets = state.tickets.read().await;
    tickets
        .iter()
        .find(|t| t.id == id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}
