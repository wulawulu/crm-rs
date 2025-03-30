use crate::pb::{QueryRequestBuilder, TimeQuery, User, UserWithUnfinished};
use crate::{
    ResponseStream, ServiceResult, UserStatsService, UserUnfinishedStream,
    pb::{QueryRequest, RawQueryRequest},
};
use chrono::{DateTime, TimeZone, Utc};
use itertools::Itertools;
use prost_types::Timestamp;
use tonic::{Response, Status};
use tracing::{info, warn};

impl UserStatsService {
    pub async fn query(&self, query: QueryRequest) -> ServiceResult<ResponseStream> {
        let sql = query.to_sql(false);

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

    pub async fn query_with_unfinished(
        &self,
        query: QueryRequest,
    ) -> ServiceResult<UserUnfinishedStream> {
        let sql = query.to_sql(true);

        let Ok(ret) = sqlx::query_as::<_, UserWithUnfinished>(&sql)
            .fetch_all(&self.inner.pool)
            .await
        else {
            warn!("Failed to fetch data with query:{}", sql);
            return Err(Status::internal(format!(
                "Failed to fetch data with query:{}",
                sql
            )));
        };
        Ok(Response::new(Box::pin(futures::stream::iter(
            ret.into_iter().map(Ok),
        ))))
    }
}

fn timestamp_query(key: &str, lower: Option<&Timestamp>, upper: Option<&Timestamp>) -> String {
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

fn ts_to_utc(ts: &Timestamp) -> DateTime<Utc> {
    Utc.timestamp_opt(ts.seconds, ts.nanos as _).unwrap()
}

impl QueryRequest {
    fn to_sql(&self, with_unfinished: bool) -> String {
        let mut sql = if with_unfinished {
            "SELECT email, name,  started_but_not_finished FROM user_stats WHERE ".to_string()
        } else {
            "SELECT email, name FROM user_stats WHERE ".to_string()
        };

        let time_condition = self
            .timestamps
            .iter()
            .map(|(k, v)| timestamp_query(k, v.lower.as_ref(), v.upper.as_ref()))
            .join(" AND ");

        sql.push_str(&time_condition);

        if !self.ids.is_empty() {
            let id_conditions = self
                .ids
                .iter()
                .map(|(k, v)| ids_query(k, &v.ids))
                .join(" AND ");

            sql.push_str(" AND ");
            sql.push_str(&id_conditions);
        }

        info!("Generated SQL: {}", sql);

        sql
    }
}

impl QueryRequest {
    pub fn new_with_day(key: &str, lower: Option<i64>, upper: Option<i64>) -> Self {
        let tq = tq(lower, upper);
        QueryRequestBuilder::default()
            .timestamp((key.to_string(), tq))
            .build()
            .unwrap()
    }
}

pub fn tq(lower: Option<i64>, upper: Option<i64>) -> TimeQuery {
    TimeQuery {
        lower: lower.map(to_ts),
        upper: upper.map(to_ts),
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

fn ids_query(key: &str, ids: &[u32]) -> String {
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
    use futures::StreamExt;
    use std::collections::HashMap;

    use test_utils::id;

    use crate::pb::QueryRequestBuilder;
    use crate::test_utils;

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

    #[test]
    fn query_request_to_string_should_work() {
        let d1 = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let d2 = Utc.with_ymd_and_hms(2024, 1, 2, 0, 0, 0).unwrap();
        let mut timestamps = HashMap::new();
        let d1 = Timestamp {
            seconds: d1.timestamp(),
            nanos: d1.timestamp_subsec_nanos() as i32,
        };
        let d2 = Timestamp {
            seconds: d2.timestamp(),
            nanos: d2.timestamp_subsec_nanos() as i32,
        };
        timestamps.insert(
            "created_at".to_string(),
            TimeQuery {
                lower: Some(d1),
                upper: Some(d2),
            },
        );
        let query = QueryRequest {
            timestamps,
            ids: Default::default(),
        };
        let sql = query.to_sql(false);
        assert_eq!(
            sql,
            "SELECT email, name FROM user_stats WHERE created_at BETWEEN '2024-01-01T00:00:00+00:00' AND '2024-01-02T00:00:00+00:00'"
        );
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
}
