use std::sync::{Arc, RwLock};

use uuid::Uuid;


#[derive(Debug, Clone)]
pub struct Ticket {
    pub(crate) id: Uuid,
    pub(crate)title: String,
    description: Option<String>,
    status: String
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub tickets: Arc<RwLock<Vec<Ticket>>>
}