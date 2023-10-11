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
    /// Creates a new message with the given sender and content.
    ///
    /// #Arguments
    ///
    /// * 'sender' - A string slice holding the name or identifier of the sender.
    /// * 'content' - A string slice holding the message content.
    ///
    /// # Example
    ///
    /// ```
    /// use chat::common::message::Message;
    /// let msg = Message::new("Alice", "Hello , world!");
    /// ```
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
