#[derive(Debug, Clone, Serialize, Deserealize)]
pub struct Message{
    pub sender: String,
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
