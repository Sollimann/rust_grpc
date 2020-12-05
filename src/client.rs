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

    // creating a new Request
    let request = tonic::Request::new(
        SayRequest{
            name:String::from("anshul")
        },
    );

    // sending a request and waiting for response
    let mut response = client.send_stream(request).await?.into_inner();

    // listening to stream
    while let Some(res) = response.message().await? {
        println!("NOTE = {:?}", res);
    }

    Ok(())
}