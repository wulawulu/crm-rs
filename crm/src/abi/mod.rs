use crate::CrmService;
use crate::pb::{
    RecallRequest, RecallResponse, RemindRequest, RemindResponse, WelcomeRequest, WelcomeResponse,
};
use crm_metadata::pb::metadata_client::MetadataClient;
use crm_metadata::pb::{Content, MaterializeRequest};
use crm_notification::pb::SendRequest;
use futures::StreamExt;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::transport::Channel;
use tonic::{Response, Status};
use tracing::warn;
use user_stat::pb::QueryRequest;

const MAX_CHANNEL_BUFFER: usize = 1024;

impl CrmService {
    pub async fn welcome(
        &self,
        request: WelcomeRequest,
    ) -> Result<Response<WelcomeResponse>, Status> {
        let user_query = QueryRequest::new_with_day(
            "created_at",
            Some(request.interval as i64),
            Some((request.interval - 1) as i64),
        );
        let mut user_stats_stream = self
            .user_stats
            .clone()
            .query(user_query)
            .await?
            .into_inner();

        let contents = Arc::new(get_contents(self.metadata.clone(), request.content_ids).await?);

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

    pub async fn recall(&self, request: RecallRequest) -> Result<Response<RecallResponse>, Status> {
        let user_query = QueryRequest::new_with_day(
            "last_visited_at",
            Some(request.last_visit_interval as i64),
            Some((request.last_visit_interval - 1) as i64),
        );
        let mut user_stats_stream = self
            .user_stats
            .clone()
            .query(user_query)
            .await?
            .into_inner();

        let contents = Arc::new(get_contents(self.metadata.clone(), request.content_ids).await?);

        let (tx, rx) = mpsc::channel(MAX_CHANNEL_BUFFER);

        let sender = self.config.server.sender.clone();
        tokio::spawn(async move {
            while let Some(Ok(user)) = user_stats_stream.next().await {
                let contents = contents.clone();
                let tx = tx.clone();
                let sender = sender.clone();

                // Process user_stat
                let send_request =
                    SendRequest::new("Recall".to_string(), sender, &[user.email], &contents);
                if let Err(e) = tx.send(send_request).await {
                    warn!("Failed to send recall message: {}", e);
                }
            }
        });

        self.notification
            .clone()
            .send(ReceiverStream::new(rx))
            .await?;

        Ok(Response::new(RecallResponse { id: request.id }))
    }

    pub async fn remind(&self, request: RemindRequest) -> Result<Response<RemindResponse>, Status> {
        let user_query = QueryRequest::new_with_day(
            "last_visited_at",
            Some(request.last_visit_interval as i64),
            Some((request.last_visit_interval - 1) as i64),
        );
        let mut user_stats_stream = self
            .user_stats
            .clone()
            .query_with_unfinished(user_query)
            .await?
            .into_inner();

        let (tx, rx) = mpsc::channel(MAX_CHANNEL_BUFFER);

        let sender = self.config.server.sender.clone();
        let metadata_client = self.metadata.clone();
        tokio::spawn(async move {
            while let Some(Ok(user)) = user_stats_stream.next().await {
                let content_ids = user
                    .started_but_not_finished
                    .into_iter()
                    .map(|x| x as u32)
                    .collect::<Vec<u32>>();
                let contents = Arc::new(get_contents(metadata_client.clone(), content_ids).await?);
                let tx = tx.clone();
                let sender = sender.clone();

                // Process user_stat
                let send_request =
                    SendRequest::new("Recall".to_string(), sender, &[user.email], &contents);
                if let Err(e) = tx.send(send_request).await {
                    warn!("Failed to send recall message: {}", e);
                }
            }
            Ok::<(), anyhow::Error>(())
        });

        self.notification
            .clone()
            .send(ReceiverStream::new(rx))
            .await?;

        Ok(Response::new(RemindResponse { id: request.id }))
    }
}

async fn get_contents(
    mut metadata_client: MetadataClient<Channel>,
    vec: Vec<u32>,
) -> Result<Vec<Content>, Status> {
    let content_stream = metadata_client
        .materialize(MaterializeRequest::new_with_ids(vec))
        .await?
        .into_inner();

    let contents: Vec<Content> = content_stream
        .filter_map(|v| async move { v.ok() })
        .collect()
        .await;
    Ok(contents)
}
