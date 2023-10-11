mod shared;
mod server;
mod client;

fn main() {
    // Check command line arguments to determine mode
    
    let args: Vec<String> = std::env::args().collect(); 

    if args.len() < 2 {
        println!("Usage: <binary_name> [server|client]");
        return;
    }

    match args[1].as_str() {
        "server" => {
            println!("Starting server. . . ");

            // Asynchronous code needs a runtime like Tokio to run.
            // The block_on function is a way to run async code from synchronous code.
            tokio::runtime::Runtime::new().unwrap().block_on(server::run_server()); 
        },
        "client" => {
            println!("Starting client . . .");
            tokio::runtime::Runtime::new().unwrap().block_on(client::run_client());
        },
        _ => {
            println!("Invalid option. Use 'server' to start in server mode or 'client' to start in client mode.");
        }
    }
    
}
