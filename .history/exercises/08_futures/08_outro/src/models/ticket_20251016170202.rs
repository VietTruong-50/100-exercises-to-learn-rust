use uuid::Uuid;


#[derive(Debug)]
struct Ticket {
    id: Uuid,
    title: String,
    description: Option<String>,
    status: String
}

#[derive(Debug)]