use anyhow::Result;
use crm::pb::RemindRequest;
use crm::pb::crm_client::CrmClient;
use tonic::Request;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = CrmClient::connect("http://[::1]:50000").await?;

    let request = Request::new(RemindRequest {
        id: Uuid::new_v4().to_string(),
        last_visit_interval: 30,
    });

    let result = client.remind(request).await;
    match result {
        Ok(response) => println!("RESPONSE={:?}", response),
        Err(e) => println!("ERROR={:?}", e),
    };

    Ok(())
}
