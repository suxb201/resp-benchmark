mod case;
mod config;
mod histogram;
mod random;
mod resp;

use awaitgroup::WaitGroup;
pub use case::Case;
use clap::Parser;
pub use histogram::Histogram;
pub use resp::{Client, RedisConfig};
use std::io::Write;
use std::sync::Arc;
use tokio::select;
use tokio::task::JoinSet;

#[derive(Parser)]
struct Opts {
    /// The input file to use
    #[clap(index = 1)]
    config: String,
}

async fn run_cmd(config: RedisConfig, case: impl Case) {
    let histogram = Arc::new(Histogram::new());
    let mut wg = WaitGroup::new();

    let mut tasks = JoinSet::new();
    for _ in 0..case.connections() {
        let config = config.clone();
        let histogram = histogram.clone();
        let case = case.clone();
        let worker = wg.worker();
        tasks.spawn(async move {
            let client = Client::new(config, case.requests(), histogram).await;
            worker.done();
            case.run(client).await;
        });
    }

    println!("start benchmark: {}", case.name());
    wg.wait().await; // wait all client connected
    let total_time = std::time::Instant::now();
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
        print!("\r\x1B[2Kqps: {:.0}, {}", qps, histogram);
        std::io::stdout().flush().unwrap();
        select! {
            _ = interval.tick() => {}
            _ = tasks.join_next() => {
                break;
            },
        }
    }
    let total_cnt = histogram.cnt();
    let total_time = total_time.elapsed().as_secs_f64();
    println!("\r\x1B[2Ktotal qps: {:.0}, {}", total_cnt as f64 / total_time, histogram);
    // join remaining tasks
    while !tasks.is_empty() {
        tasks.join_next().await;
    }
}

fn main() {
    let opts: Opts = Opts::parse();

    let config = config::Config::from_file(&opts.config);
    let threads = if config.threads == 0 { num_cpus::get() as u64 } else { config.threads };

    println!("Database info: {}", config.redis_config);
    println!("Threads: {}", threads);
    println!();

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
