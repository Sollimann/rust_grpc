//We would use tokio::sync::mpsc for communicating between futures
use tokio::sync::mpsc;

// gRPC tools
use tonic::{transport::Server, Request, Response, Status};

// our messages and services
use hello::say_server::{Say, SayServer};
use hello::{SayResponse, SayRequest};
mod hello;

// defining a struct for our service
#[derive(Default)]
pub struct MySay {}

// implementing rpc for service defined in .proto
#[tonic::async_trait]
impl Say for MySay {

    // create a new rpc to receive a stream
    async fn receive_stream(
        &self,
        request: Request<tonic::Streaming<SayRequest>>
    ) -> Result<Response<SayResponse>, Status>{
        // converting request into stream
        let mut stream = request.into_inner();
        let mut message = String::from("");

        // listening on stream
        while let Some(req) = stream.message().await? {
            message.push_str(&format!("Hello {}\n", req.name))
        }

        // returning response
        Ok(response::new(SayResponse {message}))
    }

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // defining address for our service
    let addr = "[::1]:50051".parse().unwrap();
    // creating a service
    let say = MySay::default();
    println!("Server listening on {}", addr);
    // adding our service to our server.
    Server::builder()
        .add_service(SayServer::new(say))
        .serve(addr)
        .await?;
    Ok(())
}