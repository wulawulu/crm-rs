use crate::pb::User;
use crate::{
    ResponseStream, ServiceResult, UserStatsService,
    pb::{QueryRequest, RawQueryRequest},
};
use chrono::{DateTime, TimeZone, Utc};
use itertools::Itertools;
use prost_types::Timestamp;
use tonic::{Response, Status};

impl UserStatsService {
    pub async fn query(&self, query: QueryRequest) -> ServiceResult<ResponseStream> {
        let mut sql = "SELECT email, name FROM user_stats WHERE ".to_string();

        let time_condition = query
            .timestamps
            .into_iter()
            .map(|(k, v)| timestamp_query(&k, v.lower, v.upper))
            .join(" AND ");

        sql.push_str(&time_condition);

        let id_conditions = query
            .ids
            .into_iter()
            .map(|(k, v)| ids_query(&k, v.ids))
            .join(" AND ");

        sql.push_str(" AND ");
        sql.push_str(&id_conditions);

        println!("Generated SQL: {}", sql);

        self.raw_query(RawQueryRequest { query: sql }).await
    }

    pub async fn raw_query(&self, req: RawQueryRequest) -> ServiceResult<ResponseStream> {
        let Ok(ret) = sqlx::query_as::<_, User>(&req.query)
            .fetch_all(&self.inner.pool)
            .await
        else {
            return Err(Status::internal(format!(
                "Failed to fetch data with query:{}",
                req.query
            )));
        };
        Ok(Response::new(Box::pin(futures::stream::iter(
            ret.into_iter().map(Ok),
        ))))
    }
}

fn timestamp_query(key: &str, lower: Option<Timestamp>, upper: Option<Timestamp>) -> String {
    match (lower, upper) {
        (Some(lower), Some(upper)) => format!(
            "{} BETWEEN '{}' AND '{}'",
            key,
            ts_to_utc(lower).to_rfc3339(),
            ts_to_utc(upper).to_rfc3339()
        ),
        (Some(lower), None) => format!("{} >= '{}'", key, ts_to_utc(lower).to_rfc3339()),
        (None, Some(upper)) => format!("{} <= '{}'", key, ts_to_utc(upper).to_rfc3339()),
        (None, None) => "TRUE".to_string(),
    }
}

fn ts_to_utc(ts: Timestamp) -> DateTime<Utc> {
    Utc.timestamp_opt(ts.seconds, ts.nanos as _).unwrap()
}

fn ids_query(key: &str, ids: Vec<u32>) -> String {
    if ids.is_empty() {
        return "TRUE".to_string();
    }
    format!("array{:?} <@ {}", ids, key)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::AppConfig;
    use anyhow::Result;
    use chrono::Utc;
    use futures::StreamExt;
    use std::time::SystemTime;

    use crate::pb::{IdQuery, QueryRequestBuilder, TimeQuery};

    #[tokio::test]
    async fn raw_query_should_work() -> Result<()> {
        let config = AppConfig::load().expect("Failed to load config");
        let svc = UserStatsService::new(config).await;
        let mut stream = svc
            .raw_query(RawQueryRequest {
                query: "select * from user_stats where created_at > '2024-01-01' limit 5"
                    .to_string(),
            })
            .await?
            .into_inner();
        while let Some(res) = stream.next().await {
            println!("{:#?}", res);
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_() -> Result<()> {
        let timestamp = Timestamp::from(SystemTime::now());
        println!("{:?}", timestamp);
        println!("{:?}", ts_to_utc(timestamp));
        println!("{:?}", ts_to_utc(timestamp).to_rfc3339());
        Ok(())
    }

    #[tokio::test]
    async fn query_should_work() -> Result<()> {
        let config = AppConfig::load().expect("Failed to load config");
        let svc = UserStatsService::new(config).await;

        let query = QueryRequestBuilder::default()
            .timestamp(("created_at".to_string(), tq(Some(120), None)))
            .timestamp(("last_visited_at".to_string(), tq(Some(30), None)))
            .id(("viewed_but_not_started".to_string(), id(&[252790])))
            .build()
            .unwrap();
        let mut stream = svc.query(query).await?.into_inner();
        while let Some(res) = stream.next().await {
            println!("{:#?}", res);
        }

        Ok(())
    }

    fn id(id: &[u32]) -> IdQuery {
        IdQuery { ids: id.to_vec() }
    }

    fn tq(lower: Option<i64>, upper: Option<i64>) -> TimeQuery {
        TimeQuery {
            lower: lower.map(to_ts),
            upper: upper.map(to_ts),
        }
    }

    fn to_ts(days: i64) -> Timestamp {
        let dt = Utc::now()
            .checked_sub_signed(chrono::Duration::days(days))
            .unwrap();
        Timestamp {
            seconds: dt.timestamp(),
            nanos: dt.timestamp_subsec_nanos() as i32,
        }
    }
}
