use uuid::Uuid;


#[de(Debug)]
struct Ticket {
    id: Uuid,
    title: String,
    description: Option<String>,
    status: String
}