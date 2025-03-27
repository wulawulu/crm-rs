use anyhow::Result;
use sqlx_db_tester::TestPg;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::time::sleep;
use tokio_stream::StreamExt;
use tonic::Request;
use tonic::codegen::tokio_stream;
use tonic::transport::Server;
use user_stat::pb::user_stats_client::UserStatsClient;
use user_stat::pb::{QueryRequestBuilder, RawQueryRequestBuilder};

use user_stat::test_utils::{id, new_for_test, tq};

const PORT_BASE: usize = 60000;

#[tokio::test]
async fn test_user_stats_query() -> Result<()> {
    let (_t_db, addr) = start_server(PORT_BASE).await?;
    let mut client = UserStatsClient::connect(format!("http://{}", addr)).await?;
    let query = QueryRequestBuilder::default()
        .timestamp(("created_at".to_string(), tq(Some(120), None)))
        .timestamp(("last_visited_at".to_string(), tq(Some(30), None)))
        .id(("viewed_but_not_started".to_string(), id(&[252790])))
        .build()
        .unwrap();
    let res = client.query(Request::new(query)).await?.into_inner();
    let ret: Vec<_> = res.map(|x| x.unwrap()).collect().await;

    assert_eq!(ret.len(), 0);

    Ok(())
}

#[tokio::test]
async fn test_user_stats_raw_query() -> Result<()> {
    let (_t_db, addr) = start_server(PORT_BASE + 1).await?;
    let mut client = UserStatsClient::connect(format!("http://{}", addr)).await?;
    let raw_query = RawQueryRequestBuilder::default()
        .query("select * from user_stats where created_at > '2024-01-01' limit 5")
        .build()?;

    let res = client.raw_query(raw_query).await?.into_inner();
    let ret: Vec<_> = res.map(|x| x.unwrap()).collect().await;

    assert_eq!(ret.len(), 5);

    Ok(())
}

async fn start_server(port: usize) -> Result<(TestPg, SocketAddr)> {
    let addr = format!("[::1]:{}", port).parse()?;

    let (tdb, svc) = new_for_test().await?;
    let svc = svc.into_server();
    tokio::spawn(async move {
        Server::builder()
            .add_service(svc)
            .serve(addr)
            .await
            .unwrap();
    });

    sleep(Duration::from_micros(1)).await;

    Ok((tdb, addr))
}
