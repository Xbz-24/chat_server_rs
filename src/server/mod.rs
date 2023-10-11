/// Core server functionalities and utilities.
pub mod core;

/// Initiates the server.
pub async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    core::start().await;
    Ok(())
}
