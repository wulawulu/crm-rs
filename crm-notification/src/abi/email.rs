use crate::NotificationService;
use crate::abi::{Sender, to_ts};
use crate::pb::send_request::Msg;
use crate::pb::{EmailMessage, SendRequest, SendResponse};
use tonic::Status;
use tracing::warn;

impl Sender for EmailMessage {
    async fn send(self, svc: NotificationService) -> Result<SendResponse, Status> {
        let message_id = self.message_id.clone();
        svc.sender.send(Msg::Email(self)).await.map_err(|e| {
            warn!("Failed to send email: {:?}", e);
            Status::internal("Failed to send email")
        })?;
        Ok(SendResponse {
            message_id,
            timestamp: Some(to_ts()),
        })
    }
}

impl From<EmailMessage> for Msg {
    fn from(msg: EmailMessage) -> Self {
        Msg::Email(msg)
    }
}

impl From<EmailMessage> for SendRequest {
    fn from(email: EmailMessage) -> Self {
        let msg: Msg = email.into();
        SendRequest { msg: Some(msg) }
    }
}

#[cfg(test)]
impl EmailMessage {
    pub fn fake() -> EmailMessage {
        use fake::Fake;
        use fake::faker::internet::en::SafeEmail;
        use uuid::Uuid;
        EmailMessage {
            message_id: Uuid::new_v4().to_string(),
            sender: SafeEmail().fake(),
            recipients: vec![SafeEmail().fake()],
            subject: "Hello".to_string(),
            body: "Hello, world!".to_string(),
        }
    }
}
