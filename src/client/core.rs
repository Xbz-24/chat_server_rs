use futures::future;
use tokio::net::TcpStream;
use crate::shared::message::Message;
use crate::shared::config;
use crate::shared::utils;
use std::io::{self, Write, Read};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::time::{SystemTime, UNIX_EPOCH};



pub async fn start() {
    let mut socket = TcpStream::connect("127.0.0.1:8080").await.unwrap();
    let (mut reader, mut writer) = socket.split();

    let reading = async {
        let mut buffer = vec![0u8; config::BUFFER_SIZE];
        loop {
            let read = reader.read(&mut buffer).await.expect("Failed to read from server");
            if read == 0 {
                return;
            }

            let received_message: Message = utils::deserialize_message(&buffer[..read])
                .expect("Failed to deserialize message");

            println!("{}: {}", received_message.sender, received_message.content);
        }
    };

    let writing = async {
        loop {
            let mut input = String::new();
            print!("You: ");
            io::stdout().flush().expect("Failed to flush stdout");
            io::stdin().read_line(&mut input).expect("Failed to read from stdin");

            let trimmed = input.trim();
            if trimmed == "/quit" {
                return;
            }

            let message = Message::new("YourUsername", trimmed);
            let serialized_msg = utils::serialize_message(&message).expect("Failed to serialize message");
            
            writer.write_all(&serialized_msg).await.expect("Failed to send message to server");
        }
    };

    tokio::select! {
        _ = reading => {},
        _ = writing => {},
    }
}
