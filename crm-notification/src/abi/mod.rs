use crate::pb::send_request::Msg;
use crate::pb::{EmailMessage, SendRequest, SendResponse};
use crate::{AppConfig, NotificationService, NotificationServiceInner};
use chrono::Utc;
use crm_metadata::Tpl;
use crm_metadata::pb::Content;
use futures::{Stream, StreamExt};
use prost_types::Timestamp;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Response, Status};
use tracing::{info, warn};
use uuid::Uuid;

mod email;
mod in_app;
mod sms;

const MAX_CHANNEL_BUFFER: usize = 1024;

pub trait Sender {
    #[allow(async_fn_in_trait)]
    async fn send(self, svc: NotificationService) -> Result<SendResponse, Status>;
}

impl NotificationService {
    pub fn new(config: AppConfig) -> Self {
        let sender = dummy_sender();
        let inner = NotificationServiceInner { config, sender };
        Self {
            inner: Arc::new(inner),
        }
    }

    pub async fn send(
        &self,
        mut stream: impl Stream<Item = Result<SendRequest, Status>> + Send + 'static + Unpin,
    ) -> Result<Response<ReceiverStream<Result<SendResponse, Status>>>, Status> {
        let (tx, rx) = mpsc::channel(MAX_CHANNEL_BUFFER);
        let svc = self.clone();
        tokio::spawn(async move {
            while let Some(Ok(req)) = stream.next().await {
                let res = match req.msg {
                    Some(Msg::Email(email)) => email.send(svc.clone()).await,
                    Some(Msg::Sms(sms)) => sms.send(svc.clone()).await,
                    Some(Msg::InApp(inapp)) => inapp.send(svc.clone()).await,
                    None => {
                        warn!("Invalid request: {:?}", req);
                        Err(Status::invalid_argument("Invalid request"))
                    }
                };
                tx.send(res).await.unwrap();
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

fn dummy_sender() -> mpsc::Sender<Msg> {
    let (tx, mut rx) = mpsc::channel(MAX_CHANNEL_BUFFER);
    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            info!("Sending message: {:?}", msg);
            sleep(Duration::from_millis(300)).await;
        }
    });
    tx
}

fn to_ts() -> Timestamp {
    let now = Utc::now();
    Timestamp {
        seconds: now.timestamp(),
        nanos: now.timestamp_subsec_nanos() as i32,
    }
}

impl SendRequest {
    pub fn new(
        subject: String,
        sender: String,
        recipients: &[String],
        contents: &[Content],
    ) -> Self {
        let tpl = Tpl(contents);
        SendRequest {
            msg: Some(Msg::Email(EmailMessage {
                message_id: Uuid::new_v4().to_string(),
                subject,
                sender,
                recipients: recipients.to_vec(),
                body: tpl.to_body(),
            })),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pb::{EmailMessage, InAppMessage, SmsMessage};
    use anyhow::Result;

    #[tokio::test]
    async fn send_should_work() -> Result<()> {
        let config = AppConfig::load()?;
        let svc = NotificationService::new(config);
        let stream = tokio_stream::iter(vec![
            Ok(EmailMessage::fake().into()),
            Ok(SmsMessage::fake().into()),
            Ok(InAppMessage::fake().into()),
        ]);

        let response = svc.send(stream).await?;
        let ret = response.into_inner().collect::<Vec<_>>().await;
        ret.iter().for_each(|m| println!("{:?}", m));
        assert_eq!(ret.len(), 3);

        Ok(())
    }
}
