use anyhow::Result;
use crm::pb::WelcomeRequest;
use crm::pb::crm_client::CrmClient;
use tonic::Request;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = CrmClient::connect("http://[::1]:50000").await?;

    let request = Request::new(WelcomeRequest {
        id: Uuid::new_v4().to_string(),
        interval: 120,
        content_ids: vec![1, 2, 3],
    });

    let result = client.welcome(request).await;
    match result {
        Ok(response) => println!("RESPONSE={:?}", response),
        Err(e) => println!("ERROR={:?}", e),
    };

    Ok(())
}
