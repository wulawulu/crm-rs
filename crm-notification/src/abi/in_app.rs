use crate::NotificationService;
use crate::abi::{Sender, to_ts};
use crate::pb::send_request::Msg;
use crate::pb::{InAppMessage, SendRequest, SendResponse};
use tonic::Status;
use tracing::warn;

impl Sender for InAppMessage {
    async fn send(self, svc: NotificationService) -> Result<SendResponse, Status> {
        let message_id = self.message_id.clone();
        svc.sender.send(Msg::InApp(self)).await.map_err(|e| {
            warn!("Failed to send inApp : {:?}", e);
            Status::internal("Failed to send inApp")
        })?;
        Ok(SendResponse {
            message_id,
            timestamp: Some(to_ts()),
        })
    }
}

impl From<InAppMessage> for Msg {
    fn from(msg: InAppMessage) -> Self {
        Msg::InApp(msg)
    }
}

impl From<InAppMessage> for SendRequest {
    fn from(email: InAppMessage) -> Self {
        let msg: Msg = email.into();
        SendRequest { msg: Some(msg) }
    }
}

#[cfg(test)]
impl InAppMessage {
    pub fn fake() -> Self {
        use uuid::Uuid;
        InAppMessage {
            message_id: Uuid::new_v4().to_string(),
            device_id: Uuid::new_v4().to_string(),
            title: "Hello".to_string(),
            body: "Hello, world!".to_string(),
        }
    }
}
