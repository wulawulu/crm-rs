pub use crate::config::AppConfig;
use crate::pb::metadata_server::{Metadata, MetadataServer};
use crate::pb::{Content, MaterializeRequest};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status, Streaming, async_trait};
use tracing::info;

pub mod abi;
mod config;
pub mod pb;

pub use abi::Tpl;

pub struct MetadataService {
    #[allow(unused)]
    config: AppConfig,
}

#[async_trait]
impl Metadata for MetadataService {
    type MaterializeStream = ReceiverStream<Result<Content, Status>>;

    async fn materialize(
        &self,
        request: Request<Streaming<MaterializeRequest>>,
    ) -> Result<Response<Self::MaterializeStream>, Status> {
        info!("receive request: {:?}", request);
        let query = request.into_inner();
        self.materialize(query).await
    }
}

impl MetadataService {
    pub fn new(config: AppConfig) -> Self {
        Self { config }
    }

    pub fn into_server(self) -> MetadataServer<Self> {
        MetadataServer::new(self)
    }
}
