use crate::shared::{config, message::Message, utils};
use std::collections::HashMap; 
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

/// Represents the connected clients and their message senders.
type Clients = HashMap<String, mpsc::UnboundedSender<Vec<u8>>>;

/// Starts the server and listens for incoming client connections.
///
/// This function binds to the specified address, starts listening for client connections,
/// and manages message broadcasting among connected clients.
pub async fn start() { 

    let listener = TcpListener::bind(config::SERVER_ADDR).await.expect("Failed to bind Server");

    println!("Server started on: {}", config::SERVER_ADDR);


    let clients: Clients = HashMap::new();

    let clients = Arc::new(Mutex::new(clients));

    loop {
        let (socket, _) = listener.accept().await.expect("Failed to accept client");

        let clients = clients.clone();

        tokio::spawn(handle_client(socket, clients));
    }

}
/// Handles the logic for a connected client.
///
/// This function reads incoming messages from a client, processes them, and broadcasts
/// them to all connected clients.
///
/// # Arguments
///
/// * `socket` - The TCPStream representing the client's connection.
/// * `clients` - A shared reference to the list of currently connected clients.
async fn handle_client(socket: TcpStream, clients: Arc<Mutex<Clients>>) {

    let mut reader = io::BufReader::new(socket);

    let (tx, mut rx) = mpsc::unbounded_channel();

    // TODO: Here you can add authentication or client logic to obtain the client's username or ID.


    let client_id = "anonymous".to_string(); // Placeholder
                                            
    {
        let mut clients = clients.lock().unwrap();
        clients.insert(client_id.clone(), tx);

    }
    let mut buffer = vec![0u8; config::BUFFER_SIZE];


    loop {

        let read = reader.read(&mut buffer).await.expect("Failed to read from client");
        if read == 0  {
            break;
        }


        let received_message: Message = utils::deserealize_message(&buffer[..read])
            .expect("Failed to desearilze message");


        // TODO: Here you can add any server-side message processing logic.
        
        
        //Broadcast to all clients 

        let clients = clients.lock().unwrap();

        for(_, client_tx) in clients.iter() {
            let serialized_msg = utils::serialize_message(&received_message)
                .expect("Failed to serialize message");
            client_tx.send(serialized_msg).unwrap();

        }
        {
            let mut clients = clients.lock().unwrap();
            clients.remove(&client_id);
    
        }

    }

}


