use uuid::Uuid;


#[derevie(Debug)]
struct Ticket {
    id: Uuid,
    title: String,
    description: Option<String>,
    status: String
}