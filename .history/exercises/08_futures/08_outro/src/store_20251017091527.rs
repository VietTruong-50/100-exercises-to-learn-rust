use crate::data::{Status, Ticket, TicketDraft, TicketId, TicketPatch};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tracing::{debug, info, warn};

#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error("Ticket with id {0} not found")]
    NotFound(TicketId),
}

#[derive(Clone)]
pub struct TicketStore {
    tickets: Arc<RwLock<HashMap<TicketId, Ticket>>>,
}

impl TicketStore {
    pub fn new() -> Self {
        info!("🏪 Initializing new ticket store");
        Self {
            tickets: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn create_ticket(&self, draft: TicketDraft) -> TicketId {
        let id = TicketId::new();
        debug!("Generated new ticket ID: {}", id);

        let ticket = Ticket {
            id,
            title: draft.title.clone(),
            description: draft.description.clone(),
            status: Status::ToDo,
        };

        let tickets = self.tickets.clone();
        tokio::task::spawn_blocking(move || {
            let mut tickets = tickets.write().unwrap();
            tickets.insert(id, ticket);
            debug!("Ticket {} stored in database", id);
        })
        .await
        .unwrap();

        info!("📦 Ticket created and stored: {}", id);
        id
    }

    pub async fn get_ticket(&self, id: TicketId) -> Result<Ticket, StoreError> {
        debug!("🔍 Looking up ticket: {}", id);

        let tickets = self.tickets.clone();
        let result = tokio::task::spawn_blocking(move || {
            let tickets = tickets.read().unwrap();
            tickets.get(&id).cloned()
        })
        .await
        .unwrap();

        match result {
            Some(ticket) => {
                debug!("✅ Ticket found: {}", id);
                Ok(ticket)
            }
            None => {
                warn!("❌ Ticket not found: {}", id);
                Err(StoreError::NotFound(id))
            }
        }
    }

    pub async fn update_ticket(
        &self,
        id: TicketId,
        patch: TicketPatch,
    ) -> Result<Ticket, StoreError> {
        debug!("🔄 Updating ticket: {}", id);

        let tickets = self.tickets.clone();
        tokio::task::spawn_blocking(move || {
            let mut tickets = tickets.write().unwrap();
            match tickets.get_mut(&id) {
                Some(ticket) => {
                    if let Some(title) = patch.title {
                        debug!("Updating title for ticket {}", id);
                        ticket.title = title;
                    }
                    if let Some(description) = patch.description {
                        debug!("Updating description for ticket {}", id);
                        ticket.description = description;
                    }
                    if let Some(status) = patch.status {
                        debug!("Updating status for ticket {} to {:?}", id, status);
                        ticket.status = status;
                    }
                    Ok(ticket.clone())
                }
                None => {
                    warn!("❌ Cannot update: ticket {} not found", id);
                    Err(StoreError::NotFound(id))
                }
            }
        })
        .await
        .unwrap()
    }

    pub async fn list_tickets(&self) -> Vec<Ticket> {
        debug!("📋 Listing all tickets");

        let tickets = self.tickets.clone();
        let result = tokio::task::spawn_blocking(move || {
            let tickets = tickets.read().unwrap();
            tickets.values().cloned().collect::<Vec<_>>()
        })
        .await
        .unwrap();

        info!("📊 Found {} tickets in store", result.len());
        result
    }
}