use crm_metadata::{AppConfig, MetadataService};
use tonic::transport::Server;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{Layer as _, fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();

    let config = AppConfig::load().expect("Failed to load config");
    let addr = config.server.port;
    let addr = format!("[::1]:{}", addr).parse().unwrap();
    info!("Metadata Service listening on {}", addr);
    let svc = MetadataService::new(config).into_server();
    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
