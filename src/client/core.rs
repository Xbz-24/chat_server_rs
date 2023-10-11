use crate::common::{config, message::Message, utils};
use std::io::{self, Write};
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn start() {
    let socket = TcpStream::connect(config::SERVER_ADDR).await.expect("Failed to connect to server");

    // Create a task to handle incoming messages
    let mut socket_clone = socket.clone();
    tokio::spawn(async move {
        let mut buffer = vec![0u8; config::BUFFER_SIZE];
        loop {
            let read = socket_clone.read(&mut buffer).await.expect("Failed to read from server");
            if read == 0 {
                break;
            }
            
            let received_message: Message = utils::deserialize_message(&buffer[..read])
                .expect("Failed to deserialize message");
            
            println!("{}: {}", received_message.sender, received_message.content);
        }
    });

    loop {
        let mut input = String::new();
        print!("You: ");
        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin().read_line(&mut input).expect("Failed to read from stdin");

        let trimmed = input.trim();
        if trimmed == "/quit" {
            break;
        }

        let message = Message::new("YourUsername", trimmed);  // Replace 'YourUsername' with actual logic to fetch the username
        let serialized_msg = utils::serialize_message(&message).expect("Failed to serialize message");
        socket.write_all(&serialized_msg).await.expect("Failed to send message to server");
    }
}

