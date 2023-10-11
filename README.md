# Rust Chat Server

A simple, yet robust chat server application written in Rust. This project utilizes asynchronous programming with Tokio, efficient message serialization with bincode and Serde, and aims to provide a GUI for client interaction.

## Features

- Asynchronous server handling, capable of managing multiple client connections simultaneously.
- Efficient message serialization for fast communication between server and client.
- A straightforward command-line interface for server management. *(Future plans to integrate a GUI for the client.)*
- Shared common structures for consistency between server and client components.

## Prerequisites

- [Rust and Cargo](https://rustup.rs/)
- [Tokio](https://tokio.rs/) for asynchronous runtime.
- [bincode](https://crates.io/crates/bincode) and [serde](https://crates.io/crates/serde) for message serialization.

## Getting Started

1. Clone the repository:
   ```bash
   git clone git@github.com:Xbz-24/chat_server_rs.git
   cd chat_server_rs
   ```

2. Run the server:
   ```bash
   cargo run --release
   ```

3. (Future) To start the client GUI, [follow additional instructions here].

## Architecture

This project is structured into three main components:

- **Server**: Handles incoming client connections, manages connected clients, and broadcasts messages.
- **Client**: Connects to the server and exchanges messages.
- **Common**: Contains shared code and structures for both server and client, ensuring consistency.

## Roadmap

- Integrate a GUI for the client application.
- Implement client authentication to allow unique usernames.
- Add private messaging capabilities.
- Enhance error handling and logging.

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under the MIT License.

---

Note: The above README assumes a basic directory and feature structure based on our discussion. You might need to make adjustments based on the actual structure and features of your project as it evolves. It also includes placeholders like "[follow additional instructions here]" that you would replace with actual content or links as your project develops.
