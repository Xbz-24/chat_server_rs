pub mod core;
pub mod gui;

pub async fn run_client() -> Result<(), Box<dyn std::error::Error>> {
    // gui::run_gui().await;
    core::start().await;
    Ok(())
}
