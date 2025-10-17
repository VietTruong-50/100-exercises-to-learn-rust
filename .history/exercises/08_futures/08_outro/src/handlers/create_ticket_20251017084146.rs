#[derive(Deserialize)]
struct CreateTicketInput {
    title: String,
    description: Option<String>,
}
