use crate::data::{Status, Ticket, TicketDraft, TicketId, TicketPatch};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

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
        Self {
            tickets: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn create_ticket(&self, draft: TicketDraft) -> TicketId {
        let id = TicketId::new();
        let ticket = Ticket {
            id,
            title: draft.title,
            description: draft.description,
            status: Status::ToDo,
        };

        let mut tickets = self.tickets.write().unwrap();
        tickets.insert(id, ticket);
        id
    }

    pub async fn get_ticket(&self, id: TicketId) -> Result<Ticket, StoreError> {
        let tickets = self.tickets.read().unwrap();
        tickets
            .get(&id)
            .cloned()
            .ok_or(StoreError::NotFound(id))
    }

    pub async fn update_ticket(
        &self,
        id: TicketId,
        patch: TicketPatch,
    ) -> Result<Ticket, StoreError> {
        let mut tickets = self.tickets.write().unwrap();
        let ticket = tickets
            .get_mut(&id)
            .ok_or(StoreError::NotFound(id))?;

        if let Some(title) = patch.title {
            ticket.title = title;
        }
        if let Some(description) = patch.description {
            ticket.description = description;
        }
        if let Some(status) = patch.status {
            ticket.status = status;
        }

        Ok(ticket.clone())
    }

    pub async fn list_tickets(&self) -> Vec<Ticket> {
        let tickets = self.tickets.read().unwrap();
        tickets.values().cloned().collect()
    }
}