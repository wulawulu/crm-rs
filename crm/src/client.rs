use anyhow::Result;
use crm::pb::CreateUserRequest;
use crm::pb::user_service_client::UserServiceClient;
use tonic::Request;

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = UserServiceClient::connect("http://[::1]:50051").await?;

    let request = Request::new(CreateUserRequest {
        name: "Alice".to_string(),
        email: "Alice@github.com".to_string(),
    });

    let response = client.create_user(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
