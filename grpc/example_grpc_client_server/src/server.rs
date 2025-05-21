use tonic::{transport::Server, Request, Response, Status};
use example::example_service_server::{ExampleService, ExampleServiceServer};
use example::ExampleMessage;

pub mod example {
    tonic::include_proto!("example");
}

#[derive(Debug, Default)]
pub struct MyExampleService;

#[tonic::async_trait]
impl ExampleService for MyExampleService {
    async fn send_message(&self, request: Request<ExampleMessage>) -> Result<Response<ExampleMessage>, Status> {
        let message = request.into_inner();
        println!("[Server] received message: {}", message.text);

        let reply = ExampleMessage {
            text: "Hello Client!".to_string(),
        };

        Ok(Response::new(reply))
    }
}

pub async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let example_service = MyExampleService::default();
    println!("Start server at [::1]:50051");
    Server::builder()
        .add_service(ExampleServiceServer::new(example_service))
        .serve(addr)
        .await?;

    Ok(())
}
