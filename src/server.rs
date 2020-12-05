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
pub struct MyGreeter {}

// implementing rpc for service defined in .proto
#[tonic::async_trait]
impl Say for MyGreeter {
    type SendStreamStream = mpsc::Receiver<Result<SayResponse, Status>>;

    async fn send_stream(
        &self,
        request: Request<SayRequest>,
    ) -> Result<Response<Self::SendStreamStream>, Status> {
        println!("Got a request from {:?}", request.remote_addr());
        let (mut tx, rx) = mpsc::channel(4);
        tokio::spawn(async move {
            for _ in 0..4 {
                tx.send(Ok(SayResponse {
                    message: format!("hello"),
                }))
                    .await;
            }
        });
        Ok(Response::new(rx))
    }

    async fn receive_stream(
        &self,
        request: Request<tonic::Streaming<SayRequest>>,
    ) -> Result<Response<SayResponse>, Status> {
        let mut stream = request.into_inner();
        let mut message = String::from("");
        while let Some(req) = stream.message().await? {
            message.push_str(&format!("Hello {}\n", req.name))
        }
        Ok(Response::new(SayResponse { message }))
    }
    async fn send(&self, request: Request<SayRequest>) -> Result<Response<SayResponse>, Status> {
        Ok(Response::new(SayResponse {
            message: format!("hello {}", request.get_ref().name),
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // defining address for our service
    let addr = "[::1]:50051".parse().unwrap();
    // creating a service
    let say = MyGreeter::default();
    println!("Server listening on {}", addr);
    // adding our service to our server.
    Server::builder()
        .add_service(SayServer::new(say))
        .serve(addr)
        .await?;
    Ok(())
}