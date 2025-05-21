use tonic::{Request};
use example::example_service_client::ExampleServiceClient;
use example::ExampleMessage;

pub mod example {
    tonic::include_proto!("example");
}

pub async fn run_client() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ExampleServiceClient::connect("http://[::1]:50051").await?;

    let request = Request::new(ExampleMessage {
        text: "Hello, server [ExampleMessage] !".to_string(),
    });

    let response = client.send_message(request).await?;

    println!("[Client] received [ExampleMessage] -> Server response: {}", response.into_inner().text);

    Ok(())
}
