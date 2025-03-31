use anyhow::Result;
use crm::pb::RemindRequest;
use crm::pb::crm_client::CrmClient;
use tonic::Request;
use tonic::transport::{Certificate, Channel, ClientTlsConfig};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<()> {
    let pem = include_str!("../../fixtures/rootCA.pem");
    let tls = ClientTlsConfig::new()
        .ca_certificate(Certificate::from_pem(pem))
        .domain_name("lili.com");
    let channel = Channel::from_static("https://[::1]:50000")
        .tls_config(tls)?
        .connect()
        .await?;
    let mut client = CrmClient::new(channel);

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
