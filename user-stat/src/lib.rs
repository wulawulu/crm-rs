use crate::config::AppConfig;
use crate::pb::user_stats_server::{UserStats, UserStatsServer};
use crate::pb::{QueryRequest, RawQueryRequest, User};
use futures::Stream;
use sqlx::PgPool;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;
use tonic::{Request, Response, Status, async_trait};

pub mod pb;

mod abi;
pub mod config;

type ServiceResult<T> = Result<Response<T>, Status>;

type ResponseStream = Pin<Box<dyn Stream<Item = Result<User, Status>> + Send>>;

#[derive(Clone)]
pub struct UserStatsService {
    inner: Arc<UserStatsServiceInner>,
}

pub struct UserStatsServiceInner {
    #[allow(unused)]
    config: AppConfig,
    pool: PgPool,
}

#[async_trait]
impl UserStats for UserStatsService {
    type QueryStream = ResponseStream;
    async fn query(&self, request: Request<QueryRequest>) -> ServiceResult<Self::QueryStream> {
        let query = request.into_inner();
        self.query(query).await
    }

    type RawQueryStream = ResponseStream;
    async fn raw_query(
        &self,
        request: Request<RawQueryRequest>,
    ) -> ServiceResult<Self::RawQueryStream> {
        let query = request.into_inner();
        self.raw_query(query).await
    }
}

impl UserStatsService {
    pub async fn new(config: AppConfig) -> Self {
        let pool = PgPool::connect(&config.server.db_url)
            .await
            .expect("Failed to connect to db");
        Self {
            inner: Arc::new(UserStatsServiceInner { config, pool }),
        }
    }

    pub fn into_server(self) -> UserStatsServer<Self> {
        UserStatsServer::new(self)
    }
}

impl Deref for UserStatsService {
    type Target = UserStatsServiceInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[cfg(feature = "test_utils")]
pub mod test_utils {
    use crate::config::AppConfig;
    use crate::pb::{IdQuery, TimeQuery};
    use crate::{UserStatsService, UserStatsServiceInner};
    use anyhow::Result;
    use chrono::Utc;
    use prost_types::Timestamp;
    use sqlx::{Executor, PgPool};
    use sqlx_db_tester::TestPg;
    use std::sync::Arc;

    pub async fn new_for_test() -> Result<(TestPg, UserStatsService)> {
        let config = AppConfig::load().expect("Failed to load config");
        let post = config.server.db_url.rfind('/').expect("invalid db_url");
        let server_url = &config.server.db_url[..post];
        let (tdb, pool) = get_test_pool(Some(server_url)).await;

        let service = UserStatsService {
            inner: Arc::new(UserStatsServiceInner { config, pool }),
        };
        Ok((tdb, service))
    }

    pub async fn get_test_pool(url: Option<&str>) -> (TestPg, PgPool) {
        let url = match url {
            None => "postgres://postgres:postgres@localhost:5432".to_string(),
            Some(url) => url.to_string(),
        };
        let tdb = TestPg::new(url, std::path::Path::new("migrations"));
        let pool = tdb.get_pool().await;

        let sql = include_str!("../fixtures/test.sql").split(";");
        let mut ts = pool.begin().await.expect("begin transaction failed");
        for s in sql {
            if s.trim().is_empty() {
                continue;
            }
            ts.execute(s).await.expect("execute sql failed");
        }
        ts.commit().await.expect("commit transaction failed");
        (tdb, pool)
    }

    pub fn id(id: &[u32]) -> IdQuery {
        IdQuery { ids: id.to_vec() }
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
}
