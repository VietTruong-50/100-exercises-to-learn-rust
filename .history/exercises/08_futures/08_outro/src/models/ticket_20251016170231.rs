use std::sync::{Arc, Mutex};

use uuid::Uuid;


#[derive(Debug)]
struct Ticket {
    id: Uuid,
    title: String,
    description: Option<String>,
    status: String
}

#[derive(Debug)]
struct AppState {
    tickets: Arc<Mutex<>>
}