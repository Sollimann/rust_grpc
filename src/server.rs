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
    // specify the output of rpc call
    type SendStreamStream=mpsc::Receiver<Result<SayResponse,Status>>;

    // our rpc impelemented as function
    async fn send_stream(&self,request:Request<SayRequest>)->Result<Response<Self::SendStreamStream>,Status>{
        println!("new request from {}",request.get_ref().name);

        // creating a queue or channel
        let (mut tx, rx) = mpsc::channel(4);

        // creating a new task
        tokio::spawn(async move {
            // looping and sending our response using streams
            for _ in 0..4{
                // sending response to our channel
                tx.send(Ok(SayResponse{
                    message: format!("hello")
                })).await;
            }
        });

        // returning a response as SayResponse message as defined in .proto
        Ok(Response::new(rx))
    }
    async fn send(&self, request: Request<SayRequest>) -> Result<Response<SayResponse>, Status> {
        Ok(Response::new(SayResponse {
            message: format!("hello {}", request.get_ref().name)
        }))
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