use crate::{
    MetadataService,
    pb::{Content, MaterializeRequest, Publisher},
};
use chrono::{DateTime, Days, Utc};
use fake::{
    Fake, Faker, Rng, faker::chrono::en::DateTimeBetween, faker::lorem::en::Sentence,
    faker::name::en::Name, rand,
};
use futures::{Stream, StreamExt, stream};
use prost_types::Timestamp;
use std::collections::HashSet;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Response, Status};

const MAX_CHANNEL_BUFFER: usize = 1024;

impl MetadataService {
    pub async fn materialize(
        &self,
        mut streaming: impl Stream<Item = Result<MaterializeRequest, Status>> + Send + 'static + Unpin,
    ) -> Result<Response<ReceiverStream<Result<Content, Status>>>, Status> {
        let (tx, rx) = mpsc::channel(MAX_CHANNEL_BUFFER);
        tokio::spawn(async move {
            while let Some(Ok(req)) = streaming.next().await {
                let content = Content::materialize(req.id);
                tx.send(Ok(content)).await.unwrap();
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

impl Content {
    pub fn materialize(id: u32) -> Self {
        let mut rng = rand::rng();
        Content {
            id,
            name: Name().fake(),
            description: Sentence(3..7).fake(),
            publishers: (1..rng.random_range(2..10))
                .map(|_| Publisher::new())
                .collect(),
            url: "https://placehold.co/1600x900".to_string(),
            image: "https://placehold.co/1600x900".to_string(),
            r#type: Faker.fake(),
            created_at: created_at(),
            views: rng.random_range(123432..10000000),
            likes: rng.random_range(1234..100000),
            dislikes: rng.random_range(123..10000),
        }
    }
}

impl Publisher {
    pub fn new() -> Self {
        Publisher {
            id: (10000..2000000).fake(),
            name: Name().fake(),
            avatar: "https://placehold.co/400x400".to_string(),
        }
    }
}

fn before(days: u64) -> DateTime<Utc> {
    Utc::now().checked_sub_days(Days::new(days)).unwrap()
}

fn created_at() -> Option<Timestamp> {
    let date: DateTime<Utc> = DateTimeBetween(before(365), before(0)).fake();
    Some(Timestamp {
        seconds: date.timestamp(),
        nanos: date.timestamp_subsec_nanos() as i32,
    })
}

pub struct Tpl<'a>(pub &'a [Content]);

impl Tpl<'_> {
    pub fn to_body(&self) -> String {
        format!("Tpl: {:?}", self.0)
    }
}

impl MaterializeRequest {
    pub fn new_with_ids(ids: Vec<u32>) -> impl Stream<Item = Self> {
        let reqs: HashSet<_> = ids.iter().map(|id| Self { id: *id }).collect();
        stream::iter(reqs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::AppConfig;
    use anyhow::Result;

    #[tokio::test]
    async fn materialize_should_work() -> Result<()> {
        let config = AppConfig::load()?;
        let service = MetadataService::new(config);
        let stream = tokio_stream::iter(vec![
            Ok(MaterializeRequest { id: 1 }),
            Ok(MaterializeRequest { id: 2 }),
            Ok(MaterializeRequest { id: 3 }),
        ]);

        let response = service.materialize(stream).await?;

        let mut stream = response.into_inner();
        let content = stream.next().await.unwrap().unwrap();
        assert_eq!(content.id, 1);
        println!("{:#?}", content);
        let content = stream.next().await.unwrap().unwrap();
        assert_eq!(content.id, 2);
        println!("{:#?}", content);
        let content = stream.next().await.unwrap().unwrap();
        assert_eq!(content.id, 3);
        println!("{:#?}", content);

        Ok(())
    }
}
