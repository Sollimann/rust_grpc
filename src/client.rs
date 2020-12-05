use::futures::stream::iter;
use hello::say_client::SayClient;
use hello::SayRequest;
mod hello;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    // creating a channel i.e connection to server
    let channel = tonic::transport::Channel::from_static("http://[::1]:50051")
        .connect()
        .await?;

    // creating a gRPC client from channel
    let mut client = SayClient::new(channel);

    // creating a stream
    let request = tonic::Request::new(iter(vec![
    SayRequest {
        name: String::from("Kristoffer"),
    },
    SayRequest {
        name: String::from("Petter"),
    },
    SayRequest {
        name: String::from("Daanky"),
    }

    ]));

    // sending stream
    let response = client.receive_stream(request).await?.into_inner();
    println!("RESPONSE=\n{}", response.message);
    Ok(())
}