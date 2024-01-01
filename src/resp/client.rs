use crate::histogram::Histogram;
use serde::Deserialize;
use std::fmt::{Display, Formatter};
use std::sync::atomic::AtomicU64;
use std::sync::Arc;

#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct RedisConfig {
    pub cluster: bool,
    pub address: String,
    pub username: String,
    pub password: String,
    pub tls: bool,
}

impl Display for RedisConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RedisConfig {{ cluster: {}, address: {}, username: {}, password: {}, tls: {} }}",
            self.cluster, self.address, self.username, self.password, self.tls
        )
    }
}

pub struct Client {
    histogram: Arc<Histogram>,
    request_count: AtomicU64,
    conn: redis::aio::Connection,
}

impl Client {
    pub async fn new(config: RedisConfig, requests: u64, histogram: Arc<Histogram>) -> Client {
        let client = redis::Client::open(format!("redis://{}", config.address)).unwrap();
        let conn = client.get_async_connection().await.unwrap();
        Client {
            histogram,
            request_count: AtomicU64::new(requests),
            conn: conn,
        }
    }

    /// Run a command pipeline.
    pub async fn run_cmd(&mut self, pipeline: &crate::resp::Pipeline) -> bool {
        let len = pipeline.len();
        if len == 0 {
            return true;
        }

        let count = self.request_count.fetch_sub(len, std::sync::atomic::Ordering::Relaxed);
        if count < len {
            return false;
        }

        let instant = std::time::Instant::now();
        pipeline.execute(&mut self.conn).await;
        let duration = instant.elapsed().as_micros() as u64;
        for _ in 0..len {
            self.histogram.record(duration);
        }
        true
    }
}
