use anyhow::Result;
use crm_metadata::pb::MaterializeRequest;
use crm_metadata::pb::metadata_client::MetadataClient;
use crm_metadata::{AppConfig, MetadataService};
use std::net::SocketAddr;
use std::time::Duration;
use tokio::time::sleep;
use tokio_stream::StreamExt;
use tonic::Request;
use tonic::transport::Server;

#[tokio::test]
async fn test_metadata() -> Result<()> {
    let addr = start_server().await?;
    let mut client = MetadataClient::connect(format!("http://{}", addr)).await?;
    let stream = tokio_stream::iter(vec![
        MaterializeRequest { id: 1 },
        MaterializeRequest { id: 2 },
        MaterializeRequest { id: 3 },
    ]);
    let res = client.materialize(Request::new(stream)).await?.into_inner();
    let ret: Vec<_> = res.map(|x| x.unwrap()).collect().await;

    assert_eq!(ret.len(), 3);

    Ok(())
}

async fn start_server() -> Result<SocketAddr> {
    let config = AppConfig::load().expect("Failed to load config");
    let addr = format!("[::1]:{}", config.server.port).parse()?;

    let svc = MetadataService::new(config).into_server();
    tokio::spawn(async move {
        Server::builder()
            .add_service(svc)
            .serve(addr)
            .await
            .unwrap();
    });

    sleep(Duration::from_micros(1)).await;

    Ok(addr)
}
