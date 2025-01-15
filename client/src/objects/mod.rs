#[derive(Default, Clone)]
pub struct TicketCase {
    pub code: String,
    pub name: String,
    pub description: String,
    pub awaiting: u32
}
impl TicketCase {
    pub fn new(code: &str, name: &str, description: &str) -> Self {
        Self {
            code: code.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            awaiting: 0
        }
    }
}