pub use crate::config::AppConfig;
use crate::pb::notification_server::{Notification, NotificationServer};
use crate::pb::send_request::Msg;
use crate::pb::{SendRequest, SendResponse};
use std::ops::Deref;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status, Streaming, async_trait};
use tracing::info;

pub mod abi;
mod config;
pub mod pb;

#[derive(Clone)]
pub struct NotificationService {
    inner: Arc<NotificationServiceInner>,
}

#[allow(unused)]
pub struct NotificationServiceInner {
    config: AppConfig,
    sender: mpsc::Sender<Msg>,
}

#[async_trait]
impl Notification for NotificationService {
    type SendStream = ReceiverStream<Result<SendResponse, Status>>;

    async fn send(
        &self,
        request: Request<Streaming<SendRequest>>,
    ) -> Result<Response<Self::SendStream>, Status> {
        info!("receive request: {:?}", request);
        let stream = request.into_inner();
        self.send(stream).await
    }
}

impl NotificationService {
    pub fn into_server(self) -> NotificationServer<Self> {
        NotificationServer::new(self)
    }
}

impl Deref for NotificationService {
    type Target = NotificationServiceInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
