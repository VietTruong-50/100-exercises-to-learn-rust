use std::sync::{Arc, RwLock};

use uuid::Uuid;


#[derive(Debug, Clone, Copy)]
struct Ticket {
    id: Uuid,
    title: String,
    description: Option<String>,
    status: String
}

#[derive(Debug)]
struct AppState {
    tickets: Arc<RwLock<Vec<Ticket>>>
}