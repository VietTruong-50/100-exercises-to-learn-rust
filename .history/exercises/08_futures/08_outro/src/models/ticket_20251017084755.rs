use std::sync::Arc;

use tokio::sync::RwLock;
use uuid::Uuid;


#[derive(Debug, Clone)]
pub struct Ticket {
    pub(crate) id: Uuid,
    pub(crate) title: String,
    pub(crate)description: Option<String>,
    pub(crate) status: String
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub tickets: Arc<RwLock<Vec<Ticket>>>
}