use std::sync::{Arc, RwLock};

use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Debug, Clone, Serialize, Deserialize)]
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