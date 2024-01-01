mod case;
mod config;
mod histogram;
mod random;
mod resp;

pub use case::Case;
pub use histogram::Histogram;
pub use resp::{Client, RedisConfig};
use std::sync::Arc;

async fn run_cmd(config: RedisConfig, case: impl Case) {
    let histogram = Arc::new(Histogram::new());

    for _ in 0..case.connections() {
        let config = config.clone();
        let histogram = histogram.clone();
        let case = case.clone();
        tokio::spawn(async move {
            let client = Client::new(config, case.requests(), histogram).await;
            case.run(client).await;
        });
    }

    let mut last_cnt = 0;
    let mut last_time = std::time::Instant::now();
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));
    loop {
        let now_cnt = histogram.cnt();
        let now_time = std::time::Instant::now();
        let duration = now_time.duration_since(last_time).as_secs_f64();
        let qps = (now_cnt - last_cnt) as f64 / duration;
        last_cnt = now_cnt;
        last_time = now_time;
        println!("\rqps: {:.0}, {}", qps, histogram);
        interval.tick().await;
    }
}

fn main() {
    const FILE: &str = "/Users/suxb201/gits/resp-benchmark/workloads/redis.toml";
    let config = config::Config::from_file(FILE);
    let threads = if config.threads == 0 { num_cpus::get() as u64 } else { config.threads };

    println!("Database info: {}", config.redis_config);
    println!("Threads: {}", threads);

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(threads as usize)
        .build()
        .unwrap();

    rt.block_on(async {
        if let Some(pings) = &config.ping {
            for ping in pings {
                run_cmd(config.clone_redis_config(), ping.clone()).await;
            }
        }
        if let Some(sets) = &config.set {
            for set in sets {
                run_cmd(config.clone_redis_config(), set.clone()).await;
            }
        }
    });
}
