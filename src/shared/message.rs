/// Represents a single message in the chat application.
#[derive(Debug, Clone, Serialize, Deserealize)]
pub struct Message{
    /// The sender's name or identifier.
    pub sender: String,
    /// The content of the message
    pub content: String,
    pub timestamp: u64, // UNIX timestamp
}

impl Message{
    pub fn new(sender: &str, content: &str) -> Self {
        Message {
            sender: sender.to_string(),
            content: content.to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
        }
    }

}
