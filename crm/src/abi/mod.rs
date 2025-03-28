use crate::CrmService;
use crate::pb::{WelcomeRequest, WelcomeResponse};
use crm_metadata::pb::{Content, MaterializeRequest};
use crm_notification::pb::SendRequest;
use futures::StreamExt;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Response, Status};
use tracing::warn;
use user_stat::pb::QueryRequest;

const MAX_CHANNEL_BUFFER: usize = 1024;

impl CrmService {
    pub async fn welcome(
        &self,
        request: WelcomeRequest,
    ) -> Result<Response<WelcomeResponse>, Status> {
        let user_query =
            QueryRequest::new_with_day("created_at", Some(request.interval as i64), Some(0));
        let mut user_stats_stream = self
            .user_stats
            .clone()
            .query(user_query)
            .await?
            .into_inner();

        let content_stream = self
            .metadata
            .clone()
            .materialize(MaterializeRequest::new_with_ids(request.content_ids))
            .await?
            .into_inner();

        let contents: Vec<Content> = content_stream
            .filter_map(|v| async move { v.ok() })
            .collect()
            .await;
        let contents = Arc::new(contents);

        let (tx, rx) = mpsc::channel(MAX_CHANNEL_BUFFER);

        let sender = self.config.server.sender.clone();
        tokio::spawn(async move {
            while let Some(Ok(user)) = user_stats_stream.next().await {
                let contents = contents.clone();
                let tx = tx.clone();
                let sender = sender.clone();

                // Process user_stat
                let send_request =
                    SendRequest::new("Welcome".to_string(), sender, &[user.email], &contents);
                if let Err(e) = tx.send(send_request).await {
                    warn!("Failed to send welcome message: {}", e);
                }
            }
        });

        self.notification
            .clone()
            .send(ReceiverStream::new(rx))
            .await?;

        Ok(Response::new(WelcomeResponse { id: request.id }))
    }
}
