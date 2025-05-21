mod server;
mod client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Run the server in a separate task
    tokio::spawn(async {
        server::run_server().await.unwrap();
    });

    // Delay to ensure the server starts up properly
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    // Run the client
    client::run_client().await?;

    Ok(())
}
