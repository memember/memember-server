/// Entry-point.
fn main() {
    // Parse environment variables from ".env" file
    dotenv::dotenv().ok();

    // Start the server.
    if let Err(e) = memember::start_server() {
        tracing::error!("Error occurred while starting the server: {}", e);
    }
}
