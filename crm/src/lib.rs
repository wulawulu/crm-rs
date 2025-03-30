mod abi;
mod config;

pub mod pb;

pub use config::AppConfig;

use anyhow::Result;
use crm_metadata::pb::metadata_client::MetadataClient;
use crm_notification::pb::notification_client::NotificationClient;
use pb::{
    RecallRequest, RecallResponse, RemindRequest, RemindResponse, WelcomeRequest, WelcomeResponse,
    crm_server::{Crm, CrmServer},
};
use tonic::{Request, Response, Status, async_trait, transport::Channel};
use tracing::info;
use user_stat::pb::user_stats_client::UserStatsClient;

pub struct CrmService {
    config: AppConfig,
    metadata: MetadataClient<Channel>,
    notification: NotificationClient<Channel>,
    user_stats: UserStatsClient<Channel>,
}

#[async_trait]
impl Crm for CrmService {
    async fn welcome(
        &self,
        request: Request<WelcomeRequest>,
    ) -> Result<Response<WelcomeResponse>, Status> {
        info!("receive request: {:?}", request);
        self.welcome(request.into_inner()).await
    }

    async fn recall(
        &self,
        request: Request<RecallRequest>,
    ) -> Result<Response<RecallResponse>, Status> {
        info!("receive request: {:?}", request);
        self.recall(request.into_inner()).await
    }

    async fn remind(
        &self,
        request: Request<RemindRequest>,
    ) -> Result<Response<RemindResponse>, Status> {
        info!("receive request: {:?}", request);
        self.remind(request.into_inner()).await
    }
}

impl CrmService {
    pub async fn try_new(config: AppConfig) -> Result<Self> {
        let metadata = MetadataClient::connect(config.server.metadata_url.clone()).await?;
        let notification =
            NotificationClient::connect(config.server.notification_url.clone()).await?;
        let user_stats = UserStatsClient::connect(config.server.user_stat_url.clone()).await?;
        Ok(Self {
            config,
            metadata,
            notification,
            user_stats,
        })
    }

    pub fn into_server(self) -> CrmServer<Self> {
        CrmServer::new(self)
    }
}
