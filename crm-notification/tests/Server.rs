use anyhow::Result;
use crm_notification::pb::notification_client::NotificationClient;
use crm_notification::pb::{EmailMessage, InAppMessage, SmsMessage};
use crm_notification::{AppConfig, NotificationService};
use std::net::SocketAddr;
use std::time::Duration;
use tokio::time::sleep;
use tokio_stream::StreamExt;
use tonic::Request;
use tonic::transport::Server;

#[tokio::test]
async fn test_notification() -> Result<()> {
    let addr = start_server().await?;
    let mut client = NotificationClient::connect(format!("http://{}", addr)).await?;
    let stream = tokio_stream::iter(vec![
        EmailMessage::fake().into(),
        SmsMessage::fake().into(),
        InAppMessage::fake().into(),
    ]);
    let res = client.send(Request::new(stream)).await?.into_inner();
    let ret: Vec<_> = res.map(|x| x.unwrap()).collect().await;

    assert_eq!(ret.len(), 3);

    Ok(())
}

async fn start_server() -> Result<SocketAddr> {
    let config = AppConfig::load().expect("Failed to load config");
    let addr = format!("[::1]:{}", config.server.port).parse()?;

    let svc = NotificationService::new(config).into_server();
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
