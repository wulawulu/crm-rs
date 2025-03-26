use crate::NotificationService;
use crate::abi::{Sender, to_ts};
use crate::pb::send_request::Msg;
use crate::pb::{SendRequest, SendResponse, SmsMessage};
use tonic::Status;
use tracing::warn;

impl Sender for SmsMessage {
    async fn send(self, svc: NotificationService) -> Result<SendResponse, Status> {
        let message_id = self.message_id.clone();
        svc.sender.send(Msg::Sms(self)).await.map_err(|e| {
            warn!("Failed to send sms : {:?}", e);
            Status::internal("Failed to send sms")
        })?;
        Ok(SendResponse {
            message_id,
            timestamp: Some(to_ts()),
        })
    }
}

impl From<SmsMessage> for Msg {
    fn from(msg: SmsMessage) -> Self {
        Msg::Sms(msg)
    }
}

impl From<SmsMessage> for SendRequest {
    fn from(email: SmsMessage) -> Self {
        let msg: Msg = email.into();
        SendRequest { msg: Some(msg) }
    }
}

#[cfg(test)]
impl SmsMessage {
    pub fn fake() -> Self {
        use fake::Fake;
        use fake::faker::phone_number::en::PhoneNumber;
        use uuid::Uuid;
        SmsMessage {
            message_id: Uuid::new_v4().to_string(),
            sender: PhoneNumber().fake(),
            recipients: vec![PhoneNumber().fake()],
            body: "Hello, world!".to_string(),
        }
    }
}
