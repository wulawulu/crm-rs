use crate::CrmService;
use crate::pb::{WelcomeRequest, WelcomeResponse};
use chrono::Utc;
use crm_metadata::pb::{Content, MaterializeRequest};
use crm_notification::pb::send_request::Msg;
use crm_notification::pb::{EmailMessage, SendRequest};
use futures::StreamExt;
use itertools::Itertools;
use prost_types::Timestamp;
use std::collections::HashMap;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Response, Status};
use user_stat::pb::{QueryRequest, TimeQuery};
use uuid::Uuid;

const MAX_CHANNEL_BUFFER: usize = 1024;

impl CrmService {
    pub async fn welcome(
        &self,
        request: WelcomeRequest,
    ) -> Result<Response<WelcomeResponse>, Status> {
        let mut timestamps = HashMap::new();
        timestamps.insert(
            "created_at".to_string(),
            TimeQuery {
                lower: Some(to_ts(request.interval as _)),
                upper: Some(to_ts(0)),
            },
        );
        let mut user_stats_stream = self
            .user_stats
            .clone()
            .query(QueryRequest {
                timestamps,
                ids: Default::default(),
            })
            .await?
            .into_inner();
        let content_stream = self
            .metadata
            .clone()
            .materialize(tokio_stream::iter(
                request
                    .content_ids
                    .into_iter()
                    .map(|id| MaterializeRequest { id }),
            ))
            .await?
            .into_inner();
        let contents: Vec<Content> = content_stream.map(|x| x.unwrap()).collect().await;
        let body = contents.into_iter().map(|x| x.name).join(",");

        let (tx, rx) = mpsc::channel(MAX_CHANNEL_BUFFER);

        let sender = self.config.server.sender.clone();

        tokio::spawn(async move {
            while let Some(Ok(user)) = user_stats_stream.next().await {
                // Process user_stat
                let send_request = SendRequest {
                    msg: Some(Msg::Email(EmailMessage {
                        message_id: Uuid::new_v4().to_string(),
                        subject: "welcome".to_string(),
                        sender: sender.clone(),
                        recipients: vec![user.email],
                        body: body.clone(),
                    })),
                };
                tx.send(send_request).await.unwrap()
            }
        });

        self.notification
            .clone()
            .send(ReceiverStream::new(rx))
            .await?;

        Ok(Response::new(WelcomeResponse { id: request.id }))
    }
}

pub fn to_ts(days: i64) -> Timestamp {
    let dt = Utc::now()
        .checked_sub_signed(chrono::Duration::days(days))
        .unwrap();
    Timestamp {
        seconds: dt.timestamp(),
        nanos: dt.timestamp_subsec_nanos() as i32,
    }
}
