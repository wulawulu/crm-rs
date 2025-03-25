use anyhow::Result;
use tonic::transport::Server;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{Layer as _, fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt};
use user_stat::UserStatsService;
use user_stat::config::AppConfig;

#[tokio::main]
async fn main() -> Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();

    let config = AppConfig::load().expect("Failed to load config");
    let addr = config.server.port;
    let addr = format!("[::1]:{}", addr).parse().unwrap();
    info!("UserService listening on {}", addr);
    let svc = UserStatsService::new(config).await.into_server();
    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
